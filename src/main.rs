use ordered_float::OrderedFloat;

#[derive(Debug)]
struct MaStruct {
    no_u32: u32,
    no_string: String,
    float: OrderedFloat<f64>,
}

fn main() {
    let s1 = MaStruct {
        no_u32: 10,
        no_string: "0A".to_string(),
        float: OrderedFloat(2.0)
    };
    let s2 = MaStruct {
        no_u32: 5,
        no_string: "BA".to_string(),
        float: OrderedFloat(1.1)
    };
    let s3 = MaStruct {
        no_u32: 100,
        no_string: "00".to_string(),
        float: OrderedFloat(3.2)
    };
    let s4 = MaStruct {
        no_u32: 9,
        no_string: "0C".to_string(),
        float: OrderedFloat(1.1)
    };
    let mut vec = vec![s1, s2, s3, s4];

    // Ne fonctionne pas avec f64 sans OrderedFloat
    /*vec.sort_by(|a, b| {
        let a_key = (&a.no_u32, &a.no_string);
        let b_key = (&b.no_u32, &b.no_string);
        a_key.cmp(&b_key)
    });*/

    vec.sort_by(|a, b| {
        let a_key = (&a.float, &a.no_string);
        let b_key = (&b.float, &b.no_string);
        a_key.partial_cmp(&b_key).unwrap()
    });

    //vec.sort_by_key(|a| a.float);
    println!("{:?}", vec);
}
