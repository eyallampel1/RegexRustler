import os

def read_and_append_file(file_path, output_file):
    with open(file_path, 'r') as file:
        output_file.write(f"{os.path.basename(file_path)}:\n")
        output_file.write(file.read())
        output_file.write("\n\n")

def main():
    src_dir = "src"
    output_file_path = "output_file.txt"

    # List all .rs files in the src directory
    files = [f for f in os.listdir(src_dir) if f.endswith('.rs')]

    # Sort files to ensure main.rs and lib.rs are processed first
    files.sort(key=lambda x: (x != "main.rs", x != "lib.rs"))

    with open(output_file_path, 'w') as output_file:
        for file_name in files:
            file_path = os.path.join(src_dir, file_name)
            read_and_append_file(file_path, output_file)

if __name__ == "__main__":
    main()
