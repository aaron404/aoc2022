


data = open("src\\7.txt").readlines()


cwd = []

class Dir:
    def __init__(self):
        self.dirs = {}
        self.files = {}

    def get_dir(self, path):
        cwd = self.dirs
        for dir in path:
            try:
                cwd = self.dirs[dir]
            except:
                self.dirs[dir] = Dir()
        return cwd
    
    def __repr__(self):
        return str({"dirs": self.dirs, "files": self.files})

fs = Dir()

for line in data:
    if line.startswith("$ cd"):
        new_dir = line.split("cd ")[1]
        if new_dir == '/':
            cwd = []
        elif new_dir == '..':
            cwd.pop()
        else:
            cwd.append(new_dir)
    elif line.startswith("$ ls"):
        pass
    else:
        a, b = line.split()
        if a == "dir":
            pass
        else:
            fs.get_dir(cwd).files[b] = a
    print(fs)



