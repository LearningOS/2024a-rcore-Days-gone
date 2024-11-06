# How to do ?
1. impl the fstat first, addding a func for the File trait
2. impl the link, only the root have directory, so we can use the root to find the file, adding a new directory to the root and modify the nlink of the disknode
3. impl the unlink, we can use the root to find the file, and then remove it, modify the nlink of the disknode, and remove the directory from the root(temporarily we overwrite the directory with a empty Directory)
4. plz pay attention to the lock of the fs, which may caused the deadlock