# Hash_Cracker
The Hash Cracker is a powerful tool written in Rust, designed to crack SHA256 hashes by comparing them with a list of passwords. It utilizes a list of 1 million common passwords, converts them into SHA256 hashes, and then compares them to the target hash.

## Getting Started

To get started with the Hash Cracker, follow these steps:
1. Clone the repository or download the Hash Cracker source code.
2. Ensure you have a compatible Rust environment installed.
3. Review the ***'passwordlist.txt'*** file. It contains a list of 1 million passwords that will be used for cracking.
   > Note: You can only modify or replace this file with your custom list of passwords if desired.
4. To run the Hash Cracker use the following command:
```
cargo run <wanted SHA256 hash>
```

## Usage

To use the Hash Cracker, follow these steps:
1. Open the terminal or command prompt.
2. Navigate to the directory where the Hash Cracker is located.
3. Use the following command:- 
```
cargo run <wanted SHA256 hash>
```
> here instead of ***'< wanted SHA256 hash >'*** insert your hash

For example:-
```
cargo run 0bb09d80600eec3eb9d7793a6f859bedde2a2d83899b70bd78e961ed674b32f4
```
4. Wait for the program to complete the cracking process. It will iterate through the list  of passwords and compare them with the target hash.
5. If the program successfully cracks the hash, it will dispaly the cracked password. If not, it will notify you that the hash could not be cracked.

## Customization
### Password List
The Hash Cracker comes with a pre-generated list of 1 million common passwords stored in the ***'passwordlist.txt'*** file. However, you can customize this list according to your needs. You can replace the contents of ***'passwordlist.txt'*** with your own list of passwords, ensuring each password is on a separate line.

### Performance Considerations
Please note that time required to crack a hash depends on various factors, including the processing power of your system, and the size of the password list. However even on a low-end system, it can check ***10,000 passwords in less than 10 seconds***. If you find the default list of passwords insufficent or want to improve the cracking speed consider using a larger, more comprehensive password list.

## Important Notes
- The Hash Cracker is designed for educational and security testing purposes only. Please use it responsibly and in compliance with the law.
- The Hash Cracker uses a list of common passwords, and it may not be successful in cracking complex or unique passwords. It is always recommended to use strong, unique passwords to ensure the security of your data.
- Be cautious when using or sharing the Hash Cracker's source code, as it can potentially be misused for unauthorized activities.

## Disclaimer
The authors and contributors of this tool are not responsible for any misuse, damages, or illegal activities performed using this software. Use it responsibly and at your own risk.

If you have any questions or encounter any issues while using the Hash Cracker, please feel free to reach out to us.

