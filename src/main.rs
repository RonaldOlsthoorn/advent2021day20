use std::{io::{BufReader, BufRead}, fs::File};
use ndarray::{Array1, Array2, s, ViewRepr, ArrayBase, Dim};
use itertools::iproduct;

const PADDING: usize = 2;
const HALD_PADDING: usize = PADDING / 2;

fn enlarge(field: &Array2<bool>, encoding: &Array1<bool>, fill: bool) -> Array2<bool> {

    let mut padded_field = Array2::<bool>::from_elem((field.nrows() + 2 * PADDING, field.ncols() + 2 * PADDING), fill);
    padded_field.slice_mut(s![PADDING as i32..-(PADDING as i32), PADDING as i32..-(PADDING as i32)]).assign(&field);

    let mut res = Array2::<bool>::default((field.nrows() + PADDING, field.ncols() + PADDING));

    for (row, col) in iproduct!(HALD_PADDING..padded_field.nrows()-HALD_PADDING, HALD_PADDING..padded_field.ncols()-HALD_PADDING) {
        let c = convolute(padded_field.slice(s![row - HALD_PADDING..row + HALD_PADDING + 1, col - HALD_PADDING..col + HALD_PADDING + 1]));
        *res.get_mut((row - HALD_PADDING, col - HALD_PADDING)).unwrap() = encoding[c];
    }

    res
}

fn convolute(subfield: ArrayBase<ViewRepr<&bool>, Dim<[usize; 2]>>) -> usize {

    let mut res = 0;

    let nelements = subfield.nrows()*subfield.ncols();

    for (b, e) in subfield.iter().enumerate() {
        if *e {
            res += 1 << (nelements - b - 1);
        }
    }

    res    
}

fn print_field(field: &Array2<bool>) {

    for row in field.rows() {
        let mut s = String::new();
        row.iter().for_each(|e| if *e {s.push('#')} else {s.push('.')});
        println!("{}", s);
    }
}


fn main() {

    let mut encoding = Array1::<bool>::default(512);

    let mut lines = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap());

    lines.next().unwrap().chars().enumerate().for_each(|(i, c)| *encoding.get_mut(i).unwrap() = c=='#');
    lines.next(); // empty line

    let field_strings: Vec<String> = lines.collect();

    let mut field = Array2::<bool>::default((field_strings.len(), field_strings[0].len()));

    field_strings.iter().enumerate().for_each(
        |(row, line)| line.chars().enumerate().for_each(
            |(column, ch)| *field.get_mut((row, column)).unwrap() = ch == '#'));

    println!("original field");
    print_field(&field);

    for (_, fill) in std::iter::zip(0..50, vec![false, true].iter().cycle()) {
        field = enlarge(&field, &encoding, *fill);
    }
    
    println!("non-zero elements: {}", field.iter().filter(|&e| *e).count());
}
