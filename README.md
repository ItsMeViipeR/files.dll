# Files

Files is a DLL that help you to manipulate files.

## Installation

To install files.dll, you have to put it in your project. Do put it as project reference in C# because it isn't a COM component.

## Usage

files.dll is very simple to use. You only need memory allocation and destruction.

```cs
// C#

#region Manipulation of memory

public static IntPtr[] AllocateString(params string[] strings)
{
    IEnumerable<IntPtr> ptrs = new List<IntPtr>();

    foreach (string str in strings)
    {
        ptrs = ptrs.Append(Marshal.StringToHGlobalAnsi(str));
    }

    return ptrs.ToArray();
}

public static void Free(params IntPtr[] ptrs)
{
    foreach (IntPtr ptr in ptrs)
    {
        Marshal.FreeHGlobal(ptr);
    }
}

#endregion

#region Import DLL functions

#region Write to file

[DllImport("path_to_files.dll")]
public static extern IntPtr read_file(IntPtr path);

[DllImport("path_to_files.dll")]
public static extern void write_file(IntPtr path, IntPtr text);

#endregion

#region Use the functions

string filePath = @"path\to\file";
string fileContent = $"thing to write in file";

// we' allocating all the memory we need for these strings
IntPtr[] ptrAllocated = AllocateString(filePath, fileContent);

// Because it's an array we can get our pointers
IntPtr filePathPtr = ptrAllocated[0];
IntPtr fileContentPtr = ptrAllocated[1];

// We write to the desired file the desired content
write_file(filePathPtr, fileContentPtr);

// We're freeing the memory allocated before
Free(filePathPtr, fileContentPtr);
#endregion

#region Read the file

// Replace with the desired file path
string filePath = @"E:\Dev\Rust\dll\test.txt";

IntPtr valueInFile = read_file(filePathPtr);

Free(valueInFile);

#endregion

#endregion
```