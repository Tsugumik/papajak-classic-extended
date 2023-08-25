# papajak-classic-extended

This is a simple program written in Rust during my studies in a technical school. It was used to decrypt answers to tests created in the "TESTAK" app, which employs a straightforward XOR-based encryption mechanism.

This program can either download the test directly from the server (for which it needs the test's name) or decrypt it from a file (in which case, it only requires a password).

## Usage

```
./executable_name [<filename_to_read:string> or <filename_to_save_download:string>] <testpassword:string> <answers_file_name:string> (optional) <testname_to_download:string>
```

Example if you want to decrypt a file

```
./executable_name example_test_file.tst example_test_password answers.txt
```

Example if you want to decrypt test from server

```
./executable_name example_file.tst example_test_password answers.txt example_test_name
```
