use std::fmt::Write;

use crate::font::{OldLayout, Header};

// -----------------------------------------------------------------------------
//     - Smush rules -
// -----------------------------------------------------------------------------

// Right has to be set
// Do not call this function in the event of right not being set
fn uni_smush(left: char, right: char, hard_blank: char) -> char {
    if right == ' ' {
        left
    } else if right == hard_blank && left == ' ' {
        left
    } else {
        right
    }
}

fn horz_smush_1(left: char, right: char, hard_blank: char) -> Option<char> {
    if left == right && left != hard_blank {
        Some(left)
    } else {
        None
    }
}

// function hRule2_Smush(ch1, ch2) {
//     var rule2Str = "|/\\[]{}()<>";
//     if (ch1 === "_") {
//         if (rule2Str.indexOf(ch2) !== -1) {return ch2;}
//     } else if (ch2 === "_") {
//         if (rule2Str.indexOf(ch1) !== -1) {return ch1;}
//     }
//     return false;
// }

fn horz_smush_2(left: char, right: char, hard_blank: char) -> Option<char> {
    let rule2str = "|/\\[]{}()<>";

    if left == '_' && rule2str.contains(right) {
        return Some(right);
    } else if right == '_' && rule2str.contains(left) {
        return Some(left);
    }

    return None;
}

// function hRule3_Smush(ch1, ch2) {
//     var rule3Classes = "| /\\ [] {} () <>";
//
//     I
//     var r3_pos1 = rule3Classes.indexOf(ch1);
//     var r3_pos2 = rule3Classes.indexOf(ch2);
//     if (r3_pos1 !== -1 && r3_pos2 !== -1) {
//         if (r3_pos1 !== r3_pos2 && Math.abs(r3_pos1-r3_pos2) !== 1) {
//             return rule3Classes.substr(Math.max(r3_pos1,r3_pos2), 1);
//         }
//     }
//     return false;
// }
fn horz_smush_3(left: char, right: char, hard_blank: char) -> Option<char> {
    let rule = "| /\\ [] {} () <>";

    match (rule.find(left), rule.find(right)) {
        (Some(left_idx), Some(right_idx)) => {
            if left_idx != right_idx && (left_idx as isize - right_idx as isize).abs() != 1 {
                let index = left_idx.max(right_idx);
                let c = rule.as_bytes()[index] as char;
                Some(c)
            } else {
                None
            }
        }
        _ => None,
    }
}

// function hRule4_Smush(ch1, ch2) {
//     var rule4Str = "[] {} ()";
//     var r4_pos1 = rule4Str.indexOf(ch1);
//     var r4_pos2 = rule4Str.indexOf(ch2);
//     if (r4_pos1 !== -1 && r4_pos2 !== -1) {
//         if (Math.abs(r4_pos1-r4_pos2) <= 1) {
//             return "|";
//         }
//     }
//     return false;
// }
fn horz_smush_4(left: char, right: char, hard_blank: char) -> Option<char> {
    let rule = "[] {} ()";
    match (rule.find(left), rule.find(right)) {
        (Some(left_idx), Some(right_idx)) => {
            if (left_idx as isize - right_idx as isize).abs() <= 1 {
                Some('|')
            } else {
                None
            }
        }
        _ => None,
    }
}

// function hRule5_Smush(ch1, ch2) {
//     var rule5Str = "/\\ \\/ ><";
//     var rule5Hash = {"0": "|", "3": "Y", "6": "X"};
//     var r5_pos1 = rule5Str.indexOf(ch1);
//     var r5_pos2 = rule5Str.indexOf(ch2);
//     if (r5_pos1 !== -1 && r5_pos2 !== -1) {
//         if ((r5_pos2-r5_pos1) === 1) {
//             return rule5Hash[r5_pos1];
//         }
//     }
//     return false;
// }
fn horz_smush_5(left: char, right: char, hard_blank: char) -> Option<char> {
    let rule = "/\\ \\/ ><";
    match (rule.find(left), rule.find(right)) {
        (Some(left_idx), Some(right_idx)) => {
            if (left_idx as isize - right_idx as isize).abs() <= 1 {
                if right_idx - left_idx == 1 {
                    return match left_idx {
                        0 => Some('|'),
                        3 => Some('Y'),
                        6 => Some('X'),
                        _ => unreachable!(),
                    };
                }
            }
        }
        _ => {}
    }

    return None;
}

// Hard blank smushing: smush two hard blanks into one
fn horz_smush_6(left: char, right: char, hard_blank: char) -> Option<char> {
    if left == hard_blank && right == hard_blank {
        Some(left)
    } else {
        None
    }
}

// -----------------------------------------------------------------------------
//     - Horizontal smush -
// -----------------------------------------------------------------------------
pub(crate) fn horizontal_smush(left: &[String], right: &[String], overlap: usize, header: &Header) -> Vec<String> {
    let debug_str = format!("{:#?}", right);
    let mut output: Vec<String> = vec!["".to_string();left.len()];

    for i in 0..header.height as usize {
        let mut left = &left[i];
        let mut right = &right[i];

        // NOTE: can this be a negative value?
        let overlap_start = (left.len() - overlap).max(0);

        // len2 = txt2.length; // <-- right.len()
        let piece1 = &left[..overlap_start];
        let mut piece2 = String::new();
        // piece2 = "";

        let seg1 = &left[overlap_start..overlap_start + overlap];
        //  var seg2 = txt2.substr(0,Math.min(overlap,len2));
        let seg2 = &right[..overlap.min(right.len())];

        for j in 0..overlap {
            // var ch1 = (jj < len1) ? seg1.substr(jj,1) : " ";
            // ch1
            let left_char = if j < left.chars().count() {
                seg1.chars().skip(j).next().unwrap()
            } else {
                ' ' 
            };

            // ch2
            let right_char = if j < right.chars().count() {
                seg2.chars().skip(j).next().unwrap()
            } else {
                ' ' 
            };

            // Smushing will take place!
            if left_char != ' ' && right_char != ' ' {
                // TODO: if FITTING layout 
                //     piece2 += uni_Smush(ch1, ch2, opts.hardBlank);
                //   else if UNIVERSAL_SMUSHING 
                //     piece2 += uni_Smush(ch1, ch2, opts.hardBlank);
                //   else  
                let mut next = None;

                if header.old_layout.contains(OldLayout::HORZ_SMUSH_1) {
                    if let Some(c) = horz_smush_1(left_char, right_char, header.hard_blank) {
                        next = Some(c);
                    }
                }

                if next.is_none() {
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_2) {
                        if let Some(c) = horz_smush_2(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }
                }

                if next.is_none() {
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_3) {
                        if let Some(c) = horz_smush_3(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }
                }

                if next.is_none() {
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_4) {
                        if let Some(c) = horz_smush_4(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }
                }

                if next.is_none() {
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_5) {
                        if let Some(c) = horz_smush_5(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }
                }


                if next.is_none() {
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_6) {
                        if let Some(c) = horz_smush_6(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }
                }

                if next.is_none() {
                    let c = uni_smush(left_char, right_char, header.hard_blank);
                    next = Some(c);
                }

                if let Some(c) = next {
                    piece2.push(c);
                }
            } else {
                let c = uni_smush(left_char, right_char, header.hard_blank);
                piece2.push(c);
            }
        }

        write!(&mut output[i], "{}", piece1);
        write!(&mut output[i], "{}", piece2);

        if overlap >= right.chars().count() {
        } else {
            let to = right.chars().count() as isize - overlap as isize;
            let piece3 = &right[overlap..overlap + to.max(0) as usize];
            write!(&mut output[i], "{}", piece3);
        }
        
    }

    output
}


// -----------------------------------------------------------------------------
//     - Horizontal smush length -
// -----------------------------------------------------------------------------
pub(crate) fn get_horizontal_smush_len(left: &str, right: &str, header: &Header) -> usize {
    if header.old_layout.contains(OldLayout::FULL_WIDTH) {
        return 0;
    }

    let max_dist = left.len();
    if max_dist == 0 {
        return 0;
    }

    let mut current_dist = 1;

    'distcal: while current_dist <= max_dist {
        // seg1 = txt1.substr(len1-curDist,curDist); // we call seg1 for left_seg
        let from = max_dist - current_dist;
        let to = from + current_dist;
        let left_seg = &left[from..to];
        let mut break_now = false;
        // let left_seg = &left[max_dist - current_dist..current_dist];
        let right_seg = &right[..right.len().min(current_dist)];

        for i in 0..right.len().min(current_dist) {
            let left_char = left_seg.chars().skip(i).next().unwrap();
            let right_char = right_seg.chars().skip(i).next().unwrap();

            if left_char != ' ' && right_char != ' ' {

                // if header.old_layout.contains(OldLayout::KERNING) {
                //     current_dist -= 1;
                //     break 'distcal;
                // }

                // TODO: add universal smushing. Requires FullLayout to be done
                // else if UniversalSmushing  (only available in Full Layout)
                    // if left_char == header.hard_blank || right_char == header.hard_blank 
                    //     current_dist -= 1;
                    // 
                // end if UniversalSmushing
                // else
                    let mut next = None;
                    break_now = true;

                    // Do all the smushing rules here
                    if header.old_layout.contains(OldLayout::HORZ_SMUSH_1) {
                        if let Some(c) = horz_smush_1(left_char, right_char, header.hard_blank) {
                            next = Some(c);
                        }
                    }

                    if next.is_none() {
                        if header.old_layout.contains(OldLayout::HORZ_SMUSH_2) {
                            if let Some(c) = horz_smush_2(left_char, right_char, header.hard_blank) {
                                next = Some(c);
                            }
                        }
                    }

                    if next.is_none() {
                        if header.old_layout.contains(OldLayout::HORZ_SMUSH_3) {
                            if let Some(c) = horz_smush_3(left_char, right_char, header.hard_blank) {
                                next = Some(c);
                            }
                        }
                    }

                    if next.is_none() {
                        if header.old_layout.contains(OldLayout::HORZ_SMUSH_4) {
                            if let Some(c) = horz_smush_4(left_char, right_char, header.hard_blank) {
                                next = Some(c);
                            }
                        }
                    }

                    if next.is_none() {
                        if header.old_layout.contains(OldLayout::HORZ_SMUSH_5) {
                            if let Some(c) = horz_smush_5(left_char, right_char, header.hard_blank) {
                                next = Some(c);
                            }
                        }
                    }

                    if next.is_none() {
                        if header.old_layout.contains(OldLayout::HORZ_SMUSH_6) {
                            if let Some(c) = horz_smush_6(left_char, right_char, header.hard_blank) {
                                next = Some(c);
                            }
                        }
                    }

                    if next.is_none() {
                        current_dist -= 1;
                        break 'distcal;
                    }
            }
        }

        if break_now {
            break;
        }
        current_dist += 1;
        if current_dist == 5 {
            let x = 1;
        }
    }

    let overlap = max_dist.min(current_dist);
    overlap
}
