use std::{ops::Deref, rc::Rc};

use bytes::Buf;
use clap::{Parser, command};
use itertools::Itertools;
use slint::{self, ModelRc, SharedString, StandardListViewItem, VecModel};

use nespile::{emulator::{context::{EmulationContext, RealEmulationContext}, memory::NesMemory}, parse_rom};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// Path to ROM to parse
    rom_path: String,
}

impl <T> From<&T> for CpuState
    where T: EmulationContext
{
    fn from(value: &T) -> Self {
        CpuState{
            a: format!("0x{:02x}", value.read_reg_a()).into(),
            x: format!("0x{:02x}", value.read_reg_x()).into(),
            y: format!("0x{:02x}", value.read_reg_y()).into(),
            pc: format!("${:04x}", value.read_reg_pc()).into(),
            sp: format!("${:02x}", value.read_reg_sp()).into(),

            break_command: value.read_flag_break_command(),
            carry: value.read_flag_carry(),
            decimal_mode: value.read_flag_decimal_mode(),
            interrupt_disable: value.read_flag_interrupt_disable(),
            negative: value.read_flag_negative(),
            overflow: value.read_flag_overflow(),
            zero: value.read_flag_zero(),
        }
    }
}



impl MemoryRow {
    fn new<'a, T: Iterator<Item=&'a u8>>(address: usize, values: T) -> Self {
        let vec_model: VecModel<SharedString> = values
            .map(|v| SharedString::from(format!("{:02x}", v)))
            .collect::<Vec<SharedString>>()
            .into();
        let values = ModelRc::from(Rc::new(vec_model));

        MemoryRow {
            address: SharedString::from(format!("${:04x}", address)),
            values,
        }
    }
}

fn bytes_to_sector<T: Deref<Target=[u8]>>(buf: &T, width: usize, offset: usize) -> MemorySector {
    let rows_raw = VecModel::from(
        buf.iter()
            .chunks(width)
            .into_iter()
            .enumerate()
            .map(|(idx, val)| MemoryRow::new(idx * width + offset, val))
            .collect::<Vec<MemoryRow>>()
    );
    let rows = ModelRc::from(Rc::new(rows_raw));
    MemorySector{ rows }
}

fn main() {
    let args = Program::parse();

    let rom = parse_rom(&args.rom_path)
        .expect("Failed to parse ROM file");
    let memory = NesMemory::new(rom.prgrom_data.into());
    let emulator = RealEmulationContext::new(memory);

    let main_window = MainWindow::new().expect("Error building main window");
    main_window.window().set_size(slint::LogicalSize::new(800f32, 600f32));

    main_window.global::<NesState>().set_rom_path(args.rom_path.into());
    main_window.global::<NesState>().set_cpu_state((&emulator).into());

    main_window.global::<NesState>().set_zeropage(bytes_to_sector(&emulator.memory.zeropage, 8, 0));
    main_window.global::<NesState>().set_stack(bytes_to_sector(&emulator.memory.stack, 8, 0x100));

    main_window.run().expect("Error running main window");
}

slint::slint!{
    import { Button, StandardListView, ListView, ScrollView } from "std-widgets.slint";

    export struct CpuState {
        a: string,
        x: string,
        y: string,
        sp: string,
        pc: string,

        carry: bool,
        zero: bool,
        interrupt_disable: bool,
        decimal_mode: bool,
        break_command: bool,
        overflow: bool,
        negative: bool,
    }

    export struct MemoryRow {
        address: string,
        values: [string],
    }
    export struct MemorySector {
        rows: [MemoryRow],
    }

    export global NesState {
        in property <string> rom_path;
        in property <CpuState> cpu_state;

        in property <MemorySector> zeropage;
        in property <MemorySector> stack;
    }

    component GeneralInfo inherits Rectangle {
        GridLayout {
            spacing: 8px;

            Row {
                Text { text: "ROM Path:"; }
                Text { text: NesState.rom_path; }
            }
        }
    }
    component EmulatorInfo inherits Rectangle {
        GridLayout {
            spacing: 4px;

            Row {
                Text {
                    text: "Registers";
                    colspan: 2;
                    font-weight: 700;
                    horizontal-alignment: center;
                }
            }
            Row {
                Text { text: "A"; }
                Text { text: NesState.cpu_state.a; }
            }
            Row {
                Text { text: "X"; }
                Text { text: NesState.cpu_state.x; }
            }
            Row {
                Text { text: "Y"; }
                Text { text: NesState.cpu_state.y; }
            }
            Row {
                Text { text: "PC"; }
                Text { text: NesState.cpu_state.pc; }
            }
            Row {
                Text { text: "SP"; }
                Text { text: NesState.cpu_state.sp; }
            }

            Row {
                Text {
                    text: "Flags";
                    colspan: 2;
                    font-weight: 700;
                    horizontal-alignment: center;
                }
            }
            Row {
                Text { text: "Negative"; }
                Text { text: NesState.cpu_state.negative ? "1" : "0"; }
            }
            Row {
                Text { text: "Overflow"; }
                Text { text: NesState.cpu_state.overflow ? "1" : "0"; }
            }
            Row {
                Text { text: "Break Command"; }
                Text { text: NesState.cpu_state.break-command ? "1" : "0"; }
            }
            Row {
                Text { text: "Decimal Mode"; }
                Text { text: NesState.cpu_state.decimal-mode ? "1" : "0"; }
            }
            Row {
                Text { text: "Interrupt Disable"; }
                Text { text: NesState.cpu_state.interrupt-disable ? "1" : "0"; }
            }
            Row {
                Text { text: "Zero"; }
                Text { text: NesState.cpu_state.zero ? "1" : "0"; }
            }
            Row {
                Text { text: "Carry"; }
                Text { text: NesState.cpu_state.carry ? "1" : "0"; }
            }
        }
    }
    component Sidebar inherits Rectangle {
        width: 40%;
        
        VerticalLayout {
            padding: 8px;
            alignment: space-between;

            VerticalLayout {
                spacing: 16px;

                GeneralInfo {}
                EmulatorInfo {}
            }
            Button {
                text: "Step";
            }
        }
    }

    component MonoText inherits Text {
        font-family: "FreeMono";
    }
    component MemoryView inherits Rectangle {
        border-width: 1px;
        border-color: black;

        in property <MemorySector> values;

        ListView {
            for row in root.values.rows : HorizontalLayout {

                MonoText {
                    text: row.address;
                }

                for value in row.values : MonoText {
                    text: value;
                }
            }
        }
    }
    component MainPanel inherits Rectangle {
        VerticalLayout {
            HorizontalLayout {
                height: 40%;

                MemoryView {
                    width: 50%;
                    values: NesState.zeropage;
                }
                MemoryView {
                    width: 50%;
                    values: NesState.stack;
                }
            }
            StandardListView {
                height: 60%;

            }
        }
    }

    export component MainWindow inherits Window {
        width: 800px;
        height: 600px;

        HorizontalLayout { 
            Sidebar {

            }
            Rectangle {
                width: 1px;
                background: black;
            }
            MainPanel {
                Text { text: "Hello, Main Panel!";}
            }
        }
    }
}