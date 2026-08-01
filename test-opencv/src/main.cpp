#include <opencv2/core.hpp>
#include <iostream>

int main() {
    cv::Mat mat = cv::Mat::eye(3, 3, CV_32F);
    std::cout << "OpenCV Identity Matrix:\n" << mat << std::endl;
    return 0;
}
