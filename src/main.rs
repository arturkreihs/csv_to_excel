fn main() {
    let data = csv_to_excel::convert("a,b,c\n1,2,3").unwrap();
    std::fs::write("output.xlsx", data).unwrap();
}
