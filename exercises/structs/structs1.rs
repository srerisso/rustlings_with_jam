// structs1.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE
// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

struct ColorClassicStruct {
    // TODO: Something goes here
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let color_name = String::from("green");
        let color = String::from("#00FF00");        
        let green = ColorClassicStruct { name: color_name, hex: color };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let color_name = String::from("green");
        let color = String::from("#00FF00");
        let green = ColorTupleStruct(color_name, color);

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // Sí, las Unit Structs, son vacías. No vacías, sino sin campos, field-less.
        // MÄQUINA. ME VOOYYY
        // El viernes mas... mañana voy a oficina y caen cervezas casi seguro
        // Como tiene que sera ... lo de las cervezas, digo. Hablamos Viernes Neeeeen. Chau.
        
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
