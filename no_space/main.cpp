#include <iostream>
#include <cassert>
#include <vector>
#include <algorithm>
#include <sstream>
#include <fstream>

#define LIM 100'000

struct File {
  std::string name;
  size_t sz;
  File(std::string n, size_t s)
    : name(n), sz(s) {}
};

struct Dir {
  std::string name;
  size_t sz;
  std::vector<Dir *> dirs;
  std::vector<File *> files;
  Dir(std::string n)
    : name(n), sz(0) {}
};

struct FileSystem {
  Dir *home;
  Dir *cdir;
  std::vector<Dir *> history;
  std::vector<size_t> deletion_candidates;

  FileSystem() {
    this->home = new Dir("/");
    cdir       = this->home;
  }

  void push_dir(std::string &name) {
    cdir->dirs.push_back(new Dir(name));
  }

  void push_file(std::string &name, size_t sz) {
    cdir->files.push_back(new File(name, sz));
  }

  void _update_dir_sizes(Dir *dir) {
    for (auto &subdir : dir->dirs) {
      subdir->sz = 0;
      this->_update_dir_sizes(subdir);
      dir->sz += subdir->sz;
    }

    for (auto &file : dir->files) {
      dir->sz += file->sz;
    }
  }

  void update_dir_sizes() {
    this->_update_dir_sizes(home);
  }

  void cd(std::string &path) {
    if (path == "/") {
      this->go_home();
    } else if (path == "..") {
      cdir = history[history.size()-1];
      history.pop_back();
    } else {
      history.push_back(cdir);
      for (auto &dir : cdir->dirs) {
        if (dir->name == path) {
          cdir = dir;
          break;
        }
      }
    }
  }

  void go_home() {
      cdir = home;
      history.clear();
  }

  void dump_dir(Dir *dir, std::string prefix, size_t *sum) {
    deletion_candidates.push_back(dir->sz);
    if (dir->sz <= LIM) {
      *sum += dir->sz;
    }
    std::cout << prefix << "- " << dir->name
              << " (" << dir->dirs.size() << " dirs, " << dir->files.size() << " files)"
              << " (file size: " << dir->sz << ')'
              << std::endl;

    for (auto &subdir : dir->dirs) {
      this->dump_dir(subdir, prefix + "  ", sum);
    }

    for (auto &file : dir->files) {
      std::cout << prefix << "  - " << file->name << " (" << file->sz << " bytes)" << std::endl;
    }
  }

  void dump_fs() {
    this->go_home();
    this->update_dir_sizes();

    size_t sum = 0;
    this->dump_dir(home, "", &sum);
    std::cout << "Sum: " << sum << std::endl;

    const size_t total_space = 70000000;
    const size_t space_needed = 30000000;

    size_t used_space = home->sz;
    size_t unused_space = total_space - used_space;

    sort(deletion_candidates.begin(), deletion_candidates.end());
    for (size_t n : deletion_candidates) {
      if (unused_space + n >= space_needed) {
        std::cout << n << std::endl;
        break;
      }
    }
  }
};

int main(void) {

  FileSystem fs;

  std::fstream file;
  file.open("input.txt", std::ios::in);

  if (file.is_open()){
    bool ls = false;
    std::string line;

    while(getline(file, line)) {
      std::stringstream ss(line);
      std::string token;
      std::string tokens[3];

      int i = 0;
      while (getline(ss, token, ' ')) {
        tokens[i++] = token;
      }

      if (tokens[0] == "$") {
        if (tokens[1] == "ls") {
          ls = true;
          continue;
        } else {
          ls = false;
        }
      }
      if (ls) {
        std::string name = tokens[1];
        if (tokens[0] == "dir") {
          fs.push_dir(name);
        } else {
          size_t sz = stoul(tokens[0]);
          fs.push_file(name, sz);
        }
      } else {
        fs.cd(tokens[2]);
      }
    }

    fs.dump_fs();

    file.close();
  }

  return 0;
}

