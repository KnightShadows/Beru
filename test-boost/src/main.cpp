#include <boost/filesystem.hpp>
#include <iostream>

int main() {
    boost::filesystem::path p("/tmp");
    if (boost::filesystem::is_directory(p)) {
        std::cout << p << " is a directory\n";
    }
    return 0;
}
