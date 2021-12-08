use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn flate_decode(content: &[u8]) -> Option<Vec<u8>> {
    let mut deflater = ZlibDecoder::new(content);
    let mut vec = vec![];
    deflater.read_to_end(&mut vec).ok()?;
    Some(vec)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keenan() {
        let data: [u8; 6274] = [
            120, 156, 197, 61, 201, 142, 93, 199, 117, 64, 150, 204, 79, 244, 42, 122, 47, 80, 223,
            212, 60, 196, 139, 68, 118, 4, 197, 145, 2, 200, 17, 29, 47, 236, 0, 110, 54, 39, 197,
            100, 55, 197, 65, 20, 243, 27, 22, 242, 169, 89, 101, 145, 115, 106, 60, 85, 117, 238,
            123, 77, 138, 114, 64, 16, 188, 188, 175, 198, 51, 79, 85, 247, 187, 11, 177, 201, 11,
            129, 127, 202, 191, 215, 207, 239, 253, 221, 191, 249, 139, 39, 175, 238, 137, 139, 47,
            224, 239, 147, 123, 223, 221, 147, 169, 193, 69, 249, 231, 250, 249, 197, 47, 239, 67,
            163, 112, 33, 205, 166, 141, 83, 23, 247, 31, 223, 203, 157, 229, 133, 116, 97, 83,
            230, 194, 25, 185, 9, 115, 113, 255, 249, 189, 223, 31, 190, 58, 138, 77, 7, 227, 132,
            62, 92, 193, 163, 243, 206, 216, 112, 184, 57, 94, 202, 45, 120, 167, 228, 225, 73,
            127, 124, 147, 31, 173, 118, 165, 173, 117, 194, 215, 6, 248, 246, 209, 241, 82, 193,
            216, 33, 202, 195, 191, 31, 165, 148, 155, 246, 154, 54, 125, 137, 77, 109, 48, 208,
            244, 219, 163, 220, 162, 11, 113, 24, 234, 117, 123, 153, 127, 246, 202, 29, 110, 251,
            240, 176, 42, 101, 213, 22, 228, 48, 232, 77, 111, 240, 16, 231, 15, 155, 211, 116, 91,
            125, 38, 210, 146, 44, 154, 236, 170, 207, 250, 10, 250, 11, 3, 192, 136, 177, 172, 10,
            223, 246, 223, 175, 113, 41, 102, 51, 198, 28, 126, 221, 55, 117, 115, 212, 113, 139,
            208, 237, 240, 253, 81, 233, 77, 169, 120, 215, 237, 123, 45, 34, 93, 223, 235, 105,
            37, 70, 25, 167, 255, 227, 254, 191, 36, 204, 194, 28, 78, 57, 131, 152, 181, 26, 6,
            20, 242, 226, 82, 69, 232, 25, 253, 197, 253, 135, 128, 212, 207, 143, 106, 243, 202,
            155, 0, 16, 1, 210, 209, 198, 7, 121, 120, 123, 68, 132, 88, 9, 51, 194, 52, 206, 105,
            88, 220, 75, 152, 198, 59, 25, 109, 2, 29, 192, 22, 134, 0, 216, 213, 222, 27, 190,
            180, 128, 207, 195, 151, 184, 31, 192, 166, 69, 28, 215, 222, 229, 209, 105, 36, 24,
            156, 71, 105, 77, 71, 135, 253, 0, 54, 132, 83, 241, 240, 41, 98, 70, 111, 1, 154, 254,
            180, 197, 125, 13, 13, 69, 106, 137, 139, 51, 184, 107, 119, 248, 6, 161, 165, 165, 48,
            0, 55, 1, 192, 209, 194, 15, 35, 61, 104, 171, 123, 150, 126, 183, 22, 168, 153, 172,
            190, 205, 148, 33, 28, 47, 252, 22, 61, 64, 21, 0, 124, 105, 225, 217, 198, 112, 113,
            41, 237, 102, 148, 208, 25, 194, 255, 116, 188, 20, 27, 52, 49, 222, 224, 64, 48, 188,
            19, 65, 134, 195, 139, 163, 222, 164, 81, 193, 194, 244, 245, 9, 208, 174, 54, 227, 12,
            0, 239, 53, 54, 181, 70, 56, 235, 14, 207, 119, 134, 184, 105, 29, 95, 227, 182, 221,
            102, 69, 0, 46, 168, 47, 31, 231, 151, 6, 30, 191, 194, 29, 64, 31, 45, 129, 158, 210,
            192, 74, 134, 58, 128, 141, 200, 187, 181, 215, 155, 246, 142, 182, 124, 149, 87, 166,
            130, 42, 43, 211, 81, 42, 83, 154, 228, 231, 235, 180, 50, 27, 85, 240, 181, 185, 133,
            158, 9, 159, 14, 240, 233, 14, 191, 37, 251, 184, 153, 166, 201, 99, 124, 159, 119,
            167, 50, 245, 180, 225, 94, 30, 237, 38, 96, 37, 145, 174, 131, 246, 123, 77, 150, 250,
            46, 179, 54, 96, 98, 130, 5, 188, 140, 48, 216, 175, 142, 151, 122, 19, 66, 70, 67, 32,
            255, 108, 103, 83, 143, 201, 192, 183, 109, 201, 47, 251, 42, 58, 14, 104, 183, 62,
            240, 167, 153, 253, 65, 194, 80, 36, 244, 133, 225, 134, 172, 0, 198, 81, 135, 207,
            142, 72, 84, 50, 184, 77, 42, 16, 194, 137, 122, 40, 134, 128, 154, 93, 144, 102, 128,
            204, 51, 178, 190, 71, 43, 2, 112, 133, 117, 5, 206, 29, 98, 155, 86, 180, 113, 69,
            123, 23, 219, 59, 219, 222, 93, 38, 122, 55, 70, 30, 100, 123, 103, 91, 59, 211, 222,
            233, 97, 195, 222, 1, 164, 101, 197, 183, 177, 192, 56, 223, 180, 6, 159, 193, 144, 10,
            152, 52, 224, 110, 63, 191, 127, 239, 55, 160, 151, 64, 68, 89, 227, 227, 133, 14, 202,
            109, 234, 194, 108, 50, 70, 165, 46, 94, 62, 186, 247, 120, 87, 111, 193, 127, 137,
            120, 107, 138, 75, 56, 0, 246, 133, 5, 41, 39, 66, 86, 92, 159, 225, 50, 92, 4, 102,
            183, 192, 223, 149, 255, 27, 132, 100, 18, 161, 89, 124, 188, 204, 156, 110, 101, 150,
            182, 89, 18, 92, 119, 241, 240, 122, 100, 255, 65, 192, 74, 191, 5, 20, 176, 247, 191,
            186, 119, 255, 111, 127, 127, 248, 103, 50, 235, 155, 52, 150, 182, 137, 151, 129, 6,
            34, 140, 86, 165, 14, 206, 133, 226, 60, 170, 13, 117, 16, 17, 59, 67, 131, 54, 192,
            147, 182, 174, 55, 156, 36, 237, 63, 63, 234, 221, 97, 175, 50, 196, 77, 122, 71, 7,
            237, 82, 19, 155, 70, 36, 82, 91, 36, 45, 246, 255, 246, 168, 144, 235, 64, 199, 127,
            127, 148, 10, 185, 82, 82, 145, 222, 123, 19, 72, 226, 239, 33, 128, 53, 17, 70, 233,
            94, 199, 124, 152, 55, 106, 3, 237, 116, 91, 95, 122, 16, 138, 181, 229, 184, 184, 50,
            208, 117, 23, 196, 223, 118, 56, 77, 243, 227, 70, 160, 237, 128, 95, 78, 212, 147,
            183, 100, 138, 140, 31, 31, 236, 170, 179, 146, 250, 33, 189, 42, 84, 93, 132, 45, 212,
            117, 63, 198, 21, 0, 249, 71, 71, 233, 166, 255, 254, 60, 49, 57, 74, 68, 169, 100,
            229, 242, 62, 105, 111, 88, 104, 194, 2, 4, 56, 168, 244, 134, 47, 250, 226, 56, 69,
            85, 86, 156, 32, 245, 45, 187, 101, 132, 148, 144, 192, 114, 142, 44, 36, 17, 23, 232,
            222, 46, 115, 69, 50, 54, 96, 81, 48, 176, 85, 21, 146, 56, 215, 131, 157, 182, 2, 117,
            163, 81, 170, 44, 198, 152, 210, 201, 75, 21, 40, 46, 113, 212, 96, 55, 175, 145, 106,
            80, 103, 7, 107, 139, 158, 215, 157, 81, 176, 229, 187, 54, 19, 66, 241, 178, 130, 241,
            82, 34, 87, 131, 234, 77, 208, 124, 76, 216, 121, 4, 87, 225, 64, 232, 7, 166, 138, 30,
            100, 192, 139, 222, 128, 48, 206, 117, 127, 36, 4, 247, 87, 40, 68, 84, 4, 217, 122,
            61, 66, 81, 123, 88, 177, 30, 169, 1, 231, 146, 176, 245, 23, 28, 183, 158, 192, 18,
            236, 113, 32, 161, 206, 237, 171, 136, 200, 67, 229, 169, 172, 41, 13, 102, 122, 239,
            92, 56, 139, 144, 68, 59, 4, 202, 79, 88, 138, 34, 192, 66, 131, 42, 162, 189, 102, 14,
            191, 59, 130, 176, 247, 176, 135, 108, 210, 3, 193, 154, 72, 166, 234, 160, 32, 27,
            172, 45, 195, 196, 187, 10, 30, 21, 216, 107, 103, 228, 13, 89, 233, 51, 4, 58, 200,
            96, 197, 219, 112, 131, 12, 72, 124, 167, 221, 6, 210, 186, 176, 221, 50, 36, 145, 49,
            89, 66, 192, 232, 26, 245, 197, 174, 52, 75, 166, 141, 112, 103, 167, 191, 229, 176,
            247, 102, 2, 78, 101, 204, 4, 28, 96, 177, 63, 86, 182, 185, 127, 12, 192, 151, 18, 28,
            131, 119, 35, 219, 103, 166, 171, 93, 64, 195, 203, 246, 251, 39, 157, 244, 250, 60,
            55, 189, 83, 22, 239, 38, 186, 2, 110, 11, 38, 243, 34, 63, 140, 161, 28, 61, 0, 126,
            22, 191, 208, 32, 243, 100, 6, 113, 98, 73, 35, 84, 134, 52, 175, 202, 0, 126, 192,
            234, 155, 2, 244, 205, 34, 78, 203, 48, 41, 48, 42, 226, 135, 161, 50, 252, 96, 36,
            112, 178, 108, 92, 129, 62, 169, 93, 232, 15, 88, 74, 112, 233, 253, 51, 237, 73, 239,
            65, 236, 192, 72, 40, 85, 204, 32, 139, 39, 165, 90, 24, 94, 7, 212, 249, 30, 173, 10,
            112, 151, 133, 160, 92, 74, 228, 5, 225, 71, 92, 104, 4, 139, 77, 142, 195, 195, 75,
            16, 118, 38, 17, 34, 14, 42, 165, 62, 199, 68, 68, 150, 223, 118, 126, 173, 67, 73, 59,
            74, 33, 13, 138, 202, 131, 119, 240, 71, 78, 107, 19, 234, 235, 44, 65, 68, 195, 32,
            170, 25, 234, 101, 249, 232, 89, 222, 169, 137, 138, 46, 245, 230, 8, 132, 170, 98, 68,
            38, 79, 12, 233, 192, 66, 172, 252, 200, 10, 70, 210, 153, 101, 66, 178, 248, 79, 112,
            74, 208, 35, 41, 62, 208, 222, 62, 101, 101, 92, 65, 132, 9, 131, 38, 35, 76, 213, 41,
            242, 217, 12, 244, 217, 41, 172, 148, 16, 35, 125, 251, 93, 31, 107, 177, 212, 38, 22,
            34, 143, 239, 50, 79, 104, 233, 40, 172, 27, 46, 51, 139, 37, 160, 93, 202, 194, 92,
            68, 73, 17, 138, 239, 16, 90, 48, 61, 111, 133, 69, 234, 194, 103, 105, 212, 42, 18,
            71, 179, 139, 12, 48, 19, 208, 98, 64, 55, 221, 70, 140, 226, 193, 216, 171, 170, 97,
            87, 102, 40, 3, 219, 119, 97, 36, 171, 134, 227, 217, 128, 117, 64, 128, 223, 115, 92,
            217, 27, 178, 134, 244, 179, 44, 63, 163, 183, 68, 95, 178, 148, 222, 141, 166, 252,
            100, 205, 142, 153, 73, 38, 39, 44, 53, 205, 137, 234, 76, 123, 214, 36, 236, 179, 119,
            226, 124, 202, 137, 166, 98, 100, 2, 162, 154, 137, 185, 171, 200, 181, 48, 27, 46,
            238, 87, 216, 192, 68, 15, 228, 122, 78, 8, 144, 125, 156, 162, 231, 180, 231, 119, 71,
            135, 14, 89, 241, 67, 75, 96, 135, 231, 206, 117, 137, 162, 68, 10, 65, 165, 88, 167,
            255, 2, 122, 12, 13, 56, 67, 129, 63, 216, 158, 153, 242, 146, 188, 31, 129, 142, 6,
            109, 81, 126, 8, 242, 209, 28, 93, 60, 153, 236, 93, 182, 71, 94, 188, 252, 87, 111,
            64, 16, 91, 44, 14, 31, 6, 74, 190, 41, 111, 227, 207, 195, 147, 252, 90, 71, 153, 149,
            162, 173, 106, 229, 62, 11, 58, 237, 189, 220, 199, 20, 186, 241, 122, 53, 255, 177,
            83, 183, 126, 38, 99, 20, 76, 13, 107, 125, 141, 201, 81, 167, 60, 0, 193, 33, 38, 178,
            83, 254, 229, 81, 155, 205, 105, 143, 184, 80, 1, 6, 208, 56, 104, 219, 222, 219, 206,
            3, 183, 20, 18, 128, 85, 231, 19, 248, 167, 249, 129, 218, 14, 127, 127, 34, 38, 0,
            155, 1, 101, 231, 42, 19, 246, 120, 233, 79, 247, 237, 83, 248, 90, 44, 76, 65, 96,
            111, 188, 122, 15, 216, 243, 76, 241, 139, 46, 5, 119, 180, 184, 50, 155, 210, 193,
            254, 100, 229, 141, 51, 89, 153, 28, 103, 182, 233, 172, 165, 179, 204, 232, 226, 143,
            19, 147, 100, 32, 178, 145, 7, 71, 70, 27, 80, 171, 254, 228, 207, 79, 122, 132, 249,
            172, 24, 155, 215, 4, 98, 168, 145, 240, 24, 253, 50, 90, 137, 143, 23, 253, 66, 43,
            168, 100, 109, 186, 67, 0, 124, 2, 82, 18, 100, 135, 194, 68, 68, 69, 251, 13, 9, 136,
            19, 183, 25, 134, 137, 134, 2, 245, 97, 107, 247, 166, 61, 17, 195, 227, 245, 132, 242,
            209, 106, 233, 179, 112, 113, 51, 147, 133, 229, 165, 18, 91, 128, 157, 207, 137, 137,
            76, 98, 74, 185, 29, 166, 65, 26, 6, 139, 201, 10, 123, 194, 100, 8, 97, 139, 240, 216,
            25, 48, 91, 252, 98, 207, 56, 152, 198, 95, 188, 184, 155, 50, 38, 200, 167, 63, 28,
            218, 222, 85, 251, 93, 76, 79, 137, 50, 98, 123, 249, 105, 113, 160, 173, 231, 21, 120,
            39, 55, 34, 167, 9, 97, 245, 16, 7, 177, 253, 231, 96, 66, 33, 204, 182, 249, 14, 210,
            191, 105, 250, 173, 3, 228, 15, 71, 92, 147, 6, 253, 55, 177, 114, 4, 172, 168, 192,
            202, 38, 20, 67, 96, 68, 99, 56, 232, 46, 6, 55, 46, 68, 130, 19, 242, 103, 106, 241,
            220, 61, 84, 145, 108, 26, 171, 55, 99, 36, 227, 49, 44, 129, 207, 26, 148, 241, 113,
            195, 220, 28, 199, 214, 153, 26, 165, 164, 228, 8, 106, 65, 97, 22, 167, 204, 240, 215,
            136, 125, 5, 140, 163, 24, 210, 117, 140, 117, 133, 113, 24, 112, 6, 48, 75, 71, 76,
            142, 91, 124, 11, 106, 74, 249, 130, 187, 61, 183, 188, 47, 147, 192, 46, 219, 35, 42,
            50, 65, 201, 201, 43, 188, 131, 127, 65, 61, 9, 45, 84, 118, 214, 213, 251, 134, 206,
            102, 231, 184, 70, 206, 170, 90, 226, 89, 145, 245, 100, 89, 14, 168, 225, 44, 51, 176,
            253, 235, 42, 197, 171, 203, 206, 134, 139, 198, 224, 118, 181, 219, 28, 37, 182, 226,
            240, 11, 195, 107, 72, 98, 136, 179, 193, 188, 135, 123, 196, 86, 4, 197, 53, 165, 135,
            130, 110, 162, 11, 95, 112, 96, 88, 28, 250, 42, 103, 208, 180, 243, 134, 202, 25, 86,
            36, 17, 91, 157, 83, 102, 205, 252, 150, 239, 227, 207, 228, 96, 53, 56, 124, 142, 117,
            210, 241, 23, 229, 202, 224, 57, 195, 58, 0, 25, 131, 185, 94, 171, 195, 15, 156, 140,
            189, 230, 20, 37, 241, 228, 234, 138, 221, 78, 84, 254, 138, 227, 133, 29, 157, 221,
            248, 43, 137, 57, 196, 162, 51, 60, 13, 78, 34, 11, 169, 205, 134, 209, 91, 206, 111,
            221, 48, 217, 171, 12, 1, 237, 205, 25, 63, 100, 9, 97, 39, 224, 142, 225, 178, 101,
            53, 107, 146, 219, 131, 178, 12, 99, 136, 32, 231, 221, 132, 202, 241, 233, 164, 161,
            199, 88, 101, 18, 64, 147, 150, 196, 165, 34, 176, 223, 213, 49, 7, 255, 112, 31, 154,
            153, 159, 199, 36, 185, 242, 32, 77, 252, 95, 34, 178, 132, 59, 21, 225, 227, 37, 70,
            222, 21, 224, 73, 172, 47, 72, 73, 75, 17, 28, 165, 10, 158, 108, 159, 230, 110, 33,
            154, 93, 91, 16, 57, 192, 105, 179, 3, 223, 51, 136, 38, 194, 229, 199, 194, 138, 224,
            190, 180, 228, 48, 145, 8, 29, 18, 221, 29, 58, 152, 246, 82, 113, 70, 8, 176, 66, 237,
            142, 172, 22, 0, 164, 114, 8, 164, 39, 237, 169, 209, 205, 25, 164, 237, 94, 44, 165,
            5, 163, 87, 174, 36, 48, 235, 30, 51, 161, 227, 110, 146, 191, 171, 32, 141, 187, 42,
            34, 145, 89, 136, 188, 151, 158, 41, 26, 235, 138, 70, 77, 87, 32, 55, 186, 230, 63,
            103, 78, 38, 43, 161, 75, 224, 19, 48, 63, 216, 248, 246, 76, 169, 221, 139, 153, 237,
            78, 222, 3, 199, 176, 35, 40, 41, 175, 229, 73, 73, 86, 50, 25, 224, 75, 111, 104, 65,
            208, 223, 193, 204, 55, 88, 57, 48, 232, 64, 173, 85, 9, 106, 51, 58, 110, 192, 46, 52,
            245, 128, 246, 1, 36, 172, 32, 229, 67, 67, 172, 189, 203, 154, 113, 152, 40, 208, 18,
            108, 71, 59, 82, 74, 154, 95, 198, 29, 43, 146, 79, 240, 34, 78, 140, 220, 172, 220,
            241, 244, 94, 149, 216, 25, 88, 231, 141, 209, 6, 77, 133, 160, 138, 59, 118, 3, 235,
            249, 78, 57, 3, 229, 129, 122, 61, 101, 222, 95, 206, 97, 135, 196, 168, 63, 148, 253,
            105, 154, 213, 1, 70, 213, 14, 220, 33, 160, 195, 51, 44, 175, 59, 75, 246, 238, 133,
            229, 65, 13, 65, 31, 235, 45, 240, 79, 168, 211, 71, 111, 134, 140, 84, 162, 24, 45,
            39, 37, 151, 40, 78, 170, 147, 246, 27, 192, 169, 68, 195, 18, 16, 71, 158, 91, 54, 59,
            145, 253, 159, 16, 63, 169, 86, 100, 202, 96, 24, 191, 97, 49, 194, 199, 140, 161, 104,
            16, 10, 128, 202, 223, 146, 82, 13, 50, 210, 135, 68, 176, 118, 67, 139, 9, 110, 202,
            76, 226, 85, 131, 33, 225, 221, 158, 247, 52, 68, 92, 160, 109, 220, 144, 89, 167, 17,
            202, 184, 188, 115, 119, 169, 45, 150, 16, 68, 102, 137, 57, 158, 134, 193, 89, 7, 86,
            46, 210, 21, 136, 98, 173, 118, 162, 47, 15, 59, 86, 200, 170, 234, 14, 118, 253, 63,
            34, 46, 8, 25, 189, 44, 216, 212, 110, 100, 215, 156, 16, 194, 244, 97, 115, 240, 248,
            232, 57, 121, 75, 140, 137, 193, 40, 215, 86, 164, 106, 47, 210, 244, 182, 236, 193,
            153, 197, 244, 156, 211, 52, 172, 111, 197, 89, 115, 197, 196, 5, 206, 251, 97, 162,
            182, 164, 225, 186, 138, 98, 211, 73, 83, 14, 41, 161, 82, 142, 70, 221, 160, 195, 10,
            108, 154, 187, 54, 47, 152, 144, 45, 27, 252, 27, 29, 35, 158, 38, 208, 186, 2, 9, 35,
            186, 214, 158, 212, 14, 97, 59, 130, 146, 158, 96, 109, 62, 119, 34, 213, 164, 172,
            157, 250, 192, 36, 213, 203, 213, 87, 230, 77, 198, 188, 233, 110, 125, 59, 98, 145,
            58, 181, 248, 23, 147, 126, 249, 135, 158, 117, 29, 235, 25, 74, 247, 29, 69, 193, 113,
            61, 169, 157, 88, 236, 184, 98, 254, 112, 139, 202, 134, 80, 164, 250, 109, 199, 87,
            46, 134, 106, 224, 203, 139, 74, 148, 4, 184, 203, 155, 94, 67, 200, 12, 52, 37, 129,
            176, 104, 11, 184, 133, 221, 253, 28, 108, 154, 194, 82, 111, 6, 28, 162, 189, 232,
            237, 162, 19, 18, 55, 16, 174, 129, 134, 1, 68, 174, 94, 129, 112, 202, 238, 131, 46,
            33, 244, 0, 10, 19, 94, 101, 35, 37, 217, 79, 213, 193, 37, 59, 183, 231, 10, 46, 11,
            156, 70, 231, 139, 75, 230, 55, 79, 208, 47, 105, 249, 146, 22, 29, 139, 91, 2, 150,
            184, 117, 55, 140, 229, 185, 215, 37, 224, 29, 86, 119, 126, 45, 89, 40, 67, 254, 137,
            99, 248, 219, 22, 32, 125, 59, 40, 178, 82, 2, 79, 9, 173, 83, 215, 127, 178, 82, 104,
            201, 10, 157, 138, 76, 80, 180, 163, 235, 235, 226, 36, 109, 115, 170, 42, 238, 235, 6,
            172, 75, 117, 82, 158, 183, 223, 246, 164, 112, 130, 32, 24, 128, 239, 235, 180, 165,
            133, 141, 190, 94, 3, 178, 39, 4, 63, 88, 190, 153, 179, 34, 137, 133, 212, 28, 177,
            13, 188, 181, 124, 197, 173, 149, 117, 239, 94, 204, 221, 167, 204, 48, 143, 13, 206,
            108, 222, 38, 18, 7, 255, 99, 177, 193, 126, 221, 150, 128, 118, 133, 75, 245, 184,
            188, 229, 189, 136, 203, 81, 133, 76, 245, 30, 14, 117, 225, 25, 95, 135, 11, 62, 241,
            65, 52, 84, 137, 78, 167, 180, 226, 100, 9, 57, 191, 41, 29, 83, 177, 140, 227, 226,
            110, 56, 16, 111, 128, 239, 5, 214, 181, 5, 161, 234, 195, 108, 220, 164, 233, 7, 6,
            237, 155, 155, 163, 20, 198, 240, 18, 255, 97, 7, 49, 199, 61, 104, 121, 69, 3, 126,
            152, 93, 68, 75, 181, 195, 157, 75, 63, 179, 65, 199, 133, 86, 247, 9, 103, 88, 83,
            159, 62, 135, 213, 113, 129, 149, 172, 71, 31, 11, 215, 174, 213, 158, 186, 249, 255,
            181, 152, 172, 79, 137, 184, 226, 75, 229, 2, 83, 68, 154, 16, 106, 118, 170, 107, 181,
            0, 19, 208, 234, 43, 110, 102, 150, 116, 19, 203, 16, 18, 32, 73, 242, 62, 7, 235, 194,
            179, 161, 4, 194, 72, 72, 26, 50, 110, 194, 77, 17, 144, 244, 114, 168, 25, 165, 85,
            113, 41, 114, 237, 44, 31, 137, 33, 179, 15, 138, 89, 171, 132, 189, 213, 14, 195, 223,
            91, 221, 202, 18, 103, 221, 77, 215, 242, 114, 154, 205, 70, 17, 208, 145, 144, 19, 58,
            160, 210, 108, 1, 246, 126, 191, 249, 46, 188, 243, 83, 189, 102, 176, 69, 168, 196,
            38, 45, 134, 0, 3, 128, 15, 15, 175, 217, 147, 81, 227, 114, 234, 168, 130, 255, 78,
            245, 98, 56, 172, 98, 92, 22, 139, 142, 17, 19, 31, 47, 209, 85, 23, 50, 78, 53, 26,
            168, 68, 254, 157, 241, 47, 219, 234, 228, 66, 92, 187, 129, 238, 186, 202, 41, 114,
            11, 224, 67, 87, 127, 201, 22, 100, 158, 64, 119, 76, 122, 160, 41, 90, 203, 185, 83,
            239, 51, 71, 247, 242, 8, 164, 212, 123, 41, 147, 118, 146, 186, 91, 153, 193, 50, 192,
            154, 27, 195, 151, 154, 45, 226, 46, 215, 205, 96, 204, 74, 168, 20, 179, 42, 102, 26,
            53, 3, 97, 167, 152, 32, 180, 131, 41, 68, 130, 90, 231, 163, 95, 202, 165, 32, 237,
            24, 89, 199, 232, 147, 0, 75, 102, 46, 55, 152, 131, 224, 15, 75, 204, 198, 232, 85,
            77, 207, 1, 239, 153, 253, 70, 153, 119, 182, 130, 108, 62, 169, 86, 53, 93, 218, 105,
            52, 107, 34, 113, 26, 234, 170, 55, 157, 180, 107, 218, 107, 96, 165, 124, 13, 104, 14,
            209, 87, 214, 32, 33, 39, 246, 146, 107, 171, 55, 47, 253, 57, 125, 49, 112, 116, 45,
            15, 109, 156, 53, 89, 107, 21, 165, 220, 152, 100, 118, 54, 244, 207, 123, 155, 115,
            66, 169, 102, 100, 51, 72, 246, 29, 19, 92, 73, 148, 43, 200, 147, 66, 35, 188, 209,
            64, 30, 217, 10, 50, 218, 116, 79, 137, 162, 35, 35, 140, 94, 28, 153, 4, 167, 209,
            143, 233, 254, 3, 161, 85, 214, 147, 237, 54, 4, 70, 223, 129, 45, 156, 247, 251, 94,
            152, 68, 200, 157, 205, 223, 114, 229, 56, 105, 116, 224, 74, 48, 221, 72, 244, 115,
            208, 151, 208, 192, 166, 216, 195, 236, 7, 207, 110, 86, 159, 116, 212, 41, 181, 156,
            241, 119, 184, 20, 139, 101, 180, 123, 1, 113, 37, 34, 24, 119, 201, 100, 72, 123, 210,
            158, 114, 210, 237, 64, 191, 196, 36, 85, 32, 199, 85, 61, 222, 210, 93, 157, 180, 175,
            240, 81, 108, 81, 37, 80, 48, 237, 7, 226, 210, 92, 150, 250, 216, 157, 5, 150, 156,
            72, 209, 36, 9, 171, 193, 181, 42, 138, 68, 133, 107, 128, 224, 227, 240, 205, 90, 136,
            203, 137, 194, 39, 101, 39, 222, 46, 40, 106, 217, 161, 178, 211, 217, 79, 75, 232,
            138, 67, 60, 161, 46, 224, 164, 95, 59, 5, 141, 70, 35, 189, 46, 230, 78, 145, 237, 2,
            192, 209, 70, 252, 87, 132, 142, 6, 10, 12, 60, 154, 31, 118, 74, 38, 42, 141, 175,
            255, 28, 79, 7, 42, 240, 16, 192, 206, 253, 224, 128, 156, 22, 120, 250, 155, 202, 116,
            140, 168, 11, 155, 156, 246, 161, 0, 51, 29, 191, 242, 113, 199, 77, 75, 191, 10, 12,
            178, 163, 137, 172, 165, 59, 169, 167, 152, 195, 9, 169, 22, 233, 196, 233, 64, 232, 2,
            236, 207, 56, 185, 107, 132, 183, 162, 142, 79, 243, 158, 235, 197, 235, 228, 84, 208,
            132, 17, 176, 233, 248, 132, 138, 24, 223, 216, 169, 182, 102, 147, 62, 3, 123, 212,
            42, 10, 63, 185, 88, 105, 250, 73, 125, 139, 176, 5, 49, 148, 112, 177, 238, 54, 63,
            21, 65, 53, 8, 117, 152, 16, 107, 52, 215, 195, 77, 61, 15, 219, 210, 107, 2, 198, 136,
            154, 177, 175, 231, 19, 2, 79, 59, 146, 118, 245, 161, 192, 250, 72, 89, 235, 188, 166,
            40, 251, 131, 108, 110, 197, 83, 14, 204, 28, 143, 39, 118, 233, 143, 19, 39, 102, 176,
            142, 170, 143, 15, 54, 179, 209, 114, 82, 252, 204, 162, 180, 184, 171, 96, 37, 205,
            238, 42, 169, 149, 65, 46, 59, 229, 174, 210, 243, 129, 102, 58, 45, 90, 75, 138, 254,
            220, 215, 252, 128, 181, 45, 217, 131, 79, 75, 69, 50, 114, 241, 143, 199, 92, 50, 63,
            30, 179, 101, 107, 191, 159, 214, 226, 168, 57, 175, 132, 112, 197, 99, 188, 252, 209,
            212, 59, 6, 115, 106, 124, 204, 208, 133, 206, 161, 160, 166, 99, 203, 140, 167, 139,
            58, 136, 134, 98, 55, 52, 201, 116, 220, 91, 42, 249, 205, 106, 79, 108, 182, 5, 239,
            222, 245, 56, 32, 91, 73, 112, 62, 228, 86, 206, 3, 156, 77, 47, 212, 205, 57, 30, 243,
            167, 178, 11, 237, 32, 237, 162, 32, 216, 226, 10, 82, 177, 68, 6, 152, 131, 139, 243,
            249, 219, 221, 83, 77, 117, 128, 18, 83, 20, 174, 88, 155, 25, 142, 205, 127, 235, 151,
            106, 144, 50, 137, 217, 63, 32, 217, 255, 244, 59, 91, 113, 49, 29, 132, 42, 238, 239,
            23, 196, 151, 59, 123, 112, 130, 231, 152, 219, 226, 128, 219, 129, 81, 174, 154, 186,
            123, 155, 165, 26, 214, 154, 81, 207, 145, 151, 178, 79, 171, 55, 127, 182, 212, 137,
            172, 235, 79, 28, 150, 7, 106, 74, 99, 170, 33, 210, 198, 7, 198, 79, 40, 156, 114,
            230, 38, 131, 206, 14, 111, 79, 196, 53, 212, 166, 178, 29, 161, 49, 34, 99, 229, 25,
            30, 92, 109, 172, 73, 183, 12, 20, 155, 203, 41, 4, 134, 212, 90, 216, 124, 208, 120,
            56, 231, 88, 196, 49, 128, 173, 106, 139, 228, 156, 139, 45, 51, 105, 82, 147, 114,
            128, 21, 91, 14, 72, 212, 60, 199, 164, 221, 203, 40, 189, 165, 9, 75, 48, 41, 67, 103,
            140, 50, 96, 188, 36, 151, 166, 230, 149, 216, 101, 168, 196, 100, 105, 168, 236, 209,
            213, 184, 20, 205, 38, 54, 3, 150, 40, 189, 41, 164, 184, 20, 85, 181, 222, 37, 188,
            47, 214, 88, 229, 249, 42, 94, 109, 205, 166, 204, 190, 147, 151, 179, 231, 246, 176,
            220, 86, 177, 220, 172, 112, 9, 34, 123, 61, 244, 94, 139, 53, 174, 202, 80, 78, 173,
            85, 111, 37, 89, 221, 4, 194, 139, 82, 226, 224, 213, 142, 245, 119, 246, 70, 7, 54,
            80, 201, 31, 86, 219, 245, 2, 210, 18, 236, 224, 222, 229, 163, 72, 46, 221, 173, 3,
            219, 1, 161, 29, 60, 173, 97, 111, 209, 211, 247, 58, 167, 203, 235, 237, 90, 182, 48,
            213, 110, 33, 140, 121, 13, 188, 170, 62, 124, 76, 229, 28, 110, 211, 211, 81, 128,
            172, 4, 211, 17, 145, 181, 112, 157, 47, 208, 96, 67, 147, 60, 171, 143, 28, 130, 253,
            141, 225, 13, 144, 231, 121, 75, 106, 181, 205, 231, 56, 8, 151, 61, 102, 53, 203, 217,
            10, 224, 132, 57, 37, 119, 170, 126, 23, 255, 46, 1, 105, 100, 43, 246, 68, 29, 209,
            71, 171, 88, 156, 174, 144, 88, 96, 53, 37, 9, 86, 61, 92, 206, 190, 107, 17, 211, 69,
            32, 35, 143, 193, 226, 140, 8, 39, 172, 240, 0, 86, 184, 91, 101, 92, 87, 44, 179, 78,
            46, 83, 129, 202, 101, 75, 75, 167, 42, 58, 228, 138, 209, 237, 27, 179, 187, 121, 1,
            225, 92, 149, 229, 251, 212, 9, 242, 1, 154, 19, 245, 236, 249, 244, 68, 18, 226, 120,
            216, 102, 208, 167, 181, 56, 61, 233, 126, 185, 133, 154, 64, 39, 33, 198, 162, 14,
            249, 197, 12, 187, 201, 90, 206, 226, 57, 193, 30, 86, 201, 158, 146, 154, 34, 32, 197,
            18, 81, 59, 193, 34, 238, 72, 27, 123, 125, 196, 24, 248, 50, 193, 96, 134, 104, 44,
            225, 168, 202, 105, 178, 190, 179, 82, 233, 220, 241, 73, 62, 229, 147, 202, 184, 178,
            194, 147, 131, 180, 36, 23, 188, 176, 90, 150, 148, 255, 142, 183, 26, 224, 246, 197,
            142, 163, 200, 26, 187, 45, 103, 141, 112, 28, 157, 58, 246, 232, 246, 136, 14, 212,
            134, 248, 59, 225, 196, 197, 135, 172, 165, 23, 90, 111, 97, 224, 142, 85, 190, 77, 7,
            117, 73, 222, 111, 58, 179, 90, 166, 101, 101, 33, 9, 250, 144, 177, 208, 249, 80, 192,
            30, 114, 13, 110, 37, 207, 109, 169, 244, 200, 184, 75, 133, 30, 0, 250, 183, 199, 4,
            36, 60, 20, 125, 167, 131, 87, 229, 200, 254, 221, 142, 223, 212, 114, 5, 60, 203, 12,
            235, 35, 162, 128, 231, 131, 41, 62, 58, 222, 211, 49, 242, 122, 58, 125, 129, 227,
            166, 171, 213, 78, 7, 40, 118, 52, 11, 13, 69, 162, 63, 157, 224, 154, 185, 207, 147,
            0, 203, 120, 165, 0, 34, 72, 251, 189, 35, 172, 107, 34, 137, 112, 217, 247, 45, 137,
            55, 70, 76, 148, 70, 126, 147, 135, 255, 46, 143, 122, 45, 13, 42, 25, 185, 12, 125,
            119, 146, 42, 203, 229, 74, 149, 42, 73, 228, 132, 183, 2, 155, 50, 60, 95, 12, 181,
            20, 200, 183, 241, 214, 104, 16, 62, 206, 1, 150, 12, 86, 122, 68, 108, 68, 219, 123,
            42, 185, 166, 99, 226, 142, 3, 55, 90, 252, 69, 2, 223, 245, 244, 30, 232, 120, 111,
            122, 225, 246, 123, 157, 222, 91, 239, 68, 91, 147, 146, 69, 62, 202, 16, 233, 158, 89,
            51, 155, 103, 139, 203, 214, 116, 39, 149, 61, 4, 46, 179, 4, 149, 59, 102, 216, 162,
            13, 39, 131, 144, 247, 94, 247, 43, 211, 75, 228, 144, 87, 86, 119, 197, 128, 84, 96,
            171, 152, 15, 66, 0, 123, 248, 101, 62, 234, 216, 224, 223, 99, 213, 61, 50, 183, 99,
            171, 48, 6, 86, 51, 218, 207, 21, 58, 16, 192, 236, 84, 6, 86, 15, 145, 115, 154, 103,
            99, 116, 97, 60, 210, 224, 92, 193, 203, 195, 130, 140, 229, 240, 229, 34, 29, 42, 143,
            41, 115, 119, 30, 59, 171, 19, 146, 52, 144, 209, 111, 106, 54, 140, 175, 89, 29, 217,
            129, 144, 79, 112, 11, 55, 29, 229, 156, 164, 225, 122, 19, 76, 42, 207, 215, 59, 231,
            0, 118, 205, 222, 84, 211, 239, 73, 65, 252, 21, 199, 168, 83, 56, 7, 189, 62, 199, 28,
            32, 175, 105, 137, 82, 251, 190, 111, 235, 22, 191, 237, 131, 238, 67, 184, 75, 226,
            36, 173, 64, 207, 46, 192, 0, 139, 19, 203, 98, 11, 216, 134, 226, 198, 129, 202, 202,
            177, 30, 118, 46, 46, 32, 196, 231, 2, 75, 117, 12, 56, 122, 190, 113, 250, 222, 33,
            215, 234, 118, 239, 8, 69, 6, 91, 225, 14, 101, 163, 167, 142, 12, 140, 209, 243, 74,
            56, 86, 146, 98, 178, 207, 200, 98, 136, 158, 94, 164, 72, 142, 142, 215, 67, 159, 61,
            171, 129, 94, 184, 113, 24, 242, 233, 39, 112, 98, 103, 88, 215, 94, 186, 254, 242,
            211, 41, 212, 154, 161, 55, 90, 194, 253, 144, 143, 232, 156, 51, 165, 193, 141, 72,
            66, 242, 235, 99, 191, 182, 129, 183, 161, 214, 162, 182, 253, 107, 47, 192, 153, 48,
            251, 114, 2, 139, 16, 28, 240, 219, 251, 167, 65, 187, 239, 119, 34, 100, 115, 83, 86,
            32, 35, 31, 243, 41, 87, 63, 99, 242, 236, 11, 130, 184, 197, 233, 157, 249, 140, 23,
            182, 83, 250, 28, 39, 30, 120, 122, 231, 146, 65, 38, 6, 188, 90, 183, 181, 0, 1, 6,
            21, 226, 35, 223, 196, 150, 45, 97, 179, 25, 197, 20, 164, 61, 235, 147, 158, 187, 99,
            133, 187, 50, 96, 199, 188, 199, 183, 22, 175, 188, 62, 127, 46, 190, 11, 159, 133, 53,
            133, 83, 189, 36, 124, 205, 2, 141, 65, 210, 115, 215, 116, 223, 45, 161, 83, 43, 195,
            19, 225, 90, 69, 161, 123, 53, 40, 83, 114, 23, 85, 141, 70, 45, 46, 107, 2, 249, 168,
            23, 167, 139, 11, 240, 246, 139, 20, 210, 122, 143, 139, 11, 82, 186, 156, 57, 82, 61,
            7, 137, 94, 28, 219, 125, 36, 76, 226, 122, 255, 184, 94, 186, 190, 195, 213, 219, 150,
            188, 157, 125, 84, 172, 59, 8, 107, 129, 70, 195, 107, 148, 96, 236, 221, 161, 216,
            141, 77, 15, 242, 49, 229, 157, 50, 229, 124, 97, 138, 86, 251, 204, 143, 107, 181,
            102, 135, 157, 248, 235, 24, 159, 229, 110, 120, 203, 196, 207, 180, 5, 242, 182, 238,
            193, 33, 41, 103, 213, 152, 238, 3, 175, 140, 58, 148, 217, 149, 171, 88, 126, 190,
            226, 182, 179, 69, 58, 45, 67, 155, 89, 14, 67, 76, 229, 30, 182, 108, 90, 89, 161,
            242, 29, 135, 169, 37, 169, 39, 61, 17, 208, 203, 65, 221, 119, 181, 222, 32, 44, 193,
            15, 36, 71, 179, 86, 105, 234, 200, 222, 170, 80, 84, 100, 134, 98, 243, 80, 249, 12,
            32, 107, 5, 142, 119, 151, 212, 223, 235, 45, 107, 113, 180, 7, 203, 189, 79, 67, 83,
            246, 110, 147, 62, 62, 151, 61, 31, 125, 127, 20, 156, 116, 154, 169, 7, 16, 76, 224,
            111, 246, 104, 199, 135, 228, 206, 81, 226, 255, 237, 189, 88, 3, 116, 206, 226, 183,
            59, 55, 210, 119, 28, 244, 185, 203, 140, 108, 123, 249, 139, 162, 85, 212, 225, 75,
            130, 205, 255, 97, 40, 151, 48, 225, 147, 174, 173, 63, 80, 155, 243, 124, 78, 110, 34,
            161, 27, 168, 153, 143, 150, 248, 32, 59, 9, 173, 93, 10, 79, 225, 41, 157, 177, 28,
            130, 59, 147, 247, 102, 65, 68, 116, 171, 29, 94, 165, 108, 82, 106, 107, 138, 114,
            225, 151, 233, 94, 199, 57, 164, 74, 239, 209, 78, 99, 14, 138, 104, 85, 212, 83, 245,
            192, 162, 32, 85, 151, 22, 69, 145, 245, 27, 201, 36, 248, 185, 23, 210, 106, 80, 141,
            119, 187, 145, 140, 126, 11, 163, 221, 71, 166, 4, 6, 143, 34, 114, 40, 254, 242, 60,
            221, 236, 133, 103, 77, 132, 139, 195, 119, 45, 174, 166, 239, 40, 228, 79, 26, 164, 2,
            146, 128, 165, 111, 253, 247, 135, 204, 211, 203, 163, 182, 24, 164, 75, 208, 22, 245,
            251, 24, 141, 66, 20, 125, 100, 90, 224, 7, 55, 242, 53, 126, 160, 68, 75, 80, 67, 147,
            237, 88, 191, 233, 74, 56, 159, 131, 240, 194, 34, 56, 149, 238, 97, 169, 143, 111,
            243, 163, 244, 57, 231, 89, 222, 190, 100, 219, 110, 233, 209, 7, 159, 24, 166, 118,
            123, 212, 223, 62, 234, 111, 111, 250, 219, 171, 249, 109, 122, 148, 189, 193, 63, 246,
            183, 79, 250, 219, 231, 203, 202, 100, 58, 231, 212, 26, 60, 235, 111, 183, 35, 234,
            66, 111, 83, 117, 72, 125, 121, 139, 47, 241, 142, 230, 62, 82, 141, 229, 16, 132, 99,
            136, 196, 77, 23, 147, 224, 103, 35, 18, 182, 101, 76, 82, 181, 98, 235, 237, 17, 47,
            205, 142, 249, 27, 46, 162, 127, 228, 35, 9, 41, 109, 26, 239, 131, 170, 248, 170, 117,
            223, 50, 201, 35, 63, 126, 121, 84, 248, 245, 27, 51, 126, 10, 227, 81, 255, 118, 72,
            255, 106, 198, 21, 247, 178, 186, 74, 48, 213, 167, 240, 100, 97, 164, 200, 96, 221,
            181, 76, 206, 171, 14, 141, 215, 44, 66, 30, 176, 224, 36, 120, 124, 201, 162, 233, 77,
            127, 75, 0, 254, 140, 157, 98, 91, 72, 197, 186, 70, 86, 125, 48, 30, 61, 14, 47, 109,
            232, 5, 191, 119, 194, 143, 203, 215, 81, 79, 248, 193, 62, 13, 63, 54, 125, 100, 7,
            112, 1, 22, 102, 197, 15, 216, 144, 223, 160, 156, 17, 30, 127, 47, 31, 102, 177, 106,
            66, 246, 3, 246, 203, 43, 5, 133, 22, 20, 62, 153, 174, 93, 90, 33, 153, 143, 238, 96,
            68, 44, 206, 134, 247, 215, 77, 196, 100, 193, 224, 114, 172, 179, 177, 253, 11, 34,
            56, 166, 239, 201, 208, 15, 227, 212, 47, 235, 100, 9, 68, 68, 72, 254, 70, 14, 136,
            90, 178, 15, 42, 204, 190, 37, 31, 239, 161, 159, 166, 161, 31, 245, 161, 235, 169, 0,
            149, 102, 104, 114, 155, 171, 185, 180, 112, 245, 219, 49, 42, 140, 131, 247, 117, 206,
            31, 240, 145, 201, 82, 46, 239, 16, 110, 233, 254, 151, 22, 151, 253, 172, 161, 125,
            134, 133, 117, 46, 214, 57, 18, 218, 146, 211, 136, 159, 17, 10, 228, 27, 48, 97, 249,
            192, 141, 67, 139, 97, 252, 188, 12, 62, 229, 46, 1, 76, 61, 65, 87, 147, 244, 204,
            111, 238, 253, 31, 36, 163, 223, 152,
        ];
        assert_eq!(flate_decode(&data).unwrap(), [b'a']);
    }
}
