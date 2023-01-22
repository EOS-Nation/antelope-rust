/**
*  Writes a number as a string
*
*  @brief Writes number x 10^(-num_decimal_places) (optionally negative) as a string
*  @param number - The number to print before shifting the decimal point to the left by num_decimal_places.
*  @param num_decimal_places - The number of decimal places to shift the decimal point.
*  @param negative - Whether to print a minus sign in the front.
*/
pub fn write_decimal(_number: u64, _num_decimal_places: u8, _negative: bool) -> String {
    let str = "".to_string();
    let _num_digits = 0;
    // let isNegative = false;

    // for ( let num of number.toString().split("").reverse() ) {
    //     if ( num == "-" ) {
    //         isNegative = true;
    //         continue;
    //     }
    //     if ( num_decimal_places != 0 && num_decimal_places == num_digits ) str = "." + str;
    //     str = num + str;
    //     num_digits += 1;
    // }

    // if ( num_digits == num_decimal_places ) str = "0." + str;
    // else if ( num_digits < num_decimal_places ) str = "0." + repeat("0", num_decimal_places - num_digits) + str;
    // else if ( str[0] == "." ) str = "0" + str;

    // if ( negative && isNegative ) str = "-" + str;
    str
}
