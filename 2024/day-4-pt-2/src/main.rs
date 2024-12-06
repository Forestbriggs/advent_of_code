// Loop through array:
// Don't check top or bottom array in array because
// there can not be an M or S above or below those respective spots
//     If hit an A:
//         Check top left and bottom right:
//             If returns "M" and "S" or "S" and "M"
//                 Check top right and bottom left:
//                     If returns "M" and "S" or "S" and "M"
//                         increment total count
