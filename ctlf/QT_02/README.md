# QT_02

Opened QtCalculator with cutter.

Decompiled (ghidra)

Found Calculator::check_result()

If the result of an operation is 4242421337.0, the flag will appear.

```c++
void Calculator::check_result()(int64_t arg1)
{
    undefined8 uVar1;
    int64_t in_FS_OFFSET;
    Calculator *this;
    int64_t var_40h;
    int64_t var_38h;
    int64_t var_30h;
    long int result;
    long int checksum;
    int64_t canary;
    
    canary = *(int64_t *)(in_FS_OFFSET + 0x28);
    result = 0;
    checksum = 0xfcde3659;
    if (*(double *)(arg1 + 0x38) == 4242421337.0) { // <----- Check is here
        QFlags<QMessageBox::StandardButton>::QFlags(QMessageBox::StandardButton)((int64_t)&var_40h, 0x400);
        QString::QString(char const*)((int64_t)&var_30h, " GG your treasure is now on the screen");
        QString::QString(char const*)((int64_t)&var_38h, "Warning");
        QMessageBox::warning(QWidget*, QString const&, QString const&, QFlags<QMessageBox::StandardButton>, QMessageBox::StandardButton)
                  (arg1, &var_38h, &var_30h, (undefined4)var_40h, 0);
        QString::~QString()((int64_t)&var_38h);
        QString::~QString()((int64_t)&var_30h);
        result = ((int64_t)(*(double *)(arg1 + 0x38) - 4200420037.0) + -0x280de5b) * 10000 + 0x539;
        Calculator::clearAll()(arg1);
        uVar1 = *(undefined8 *)(arg1 + 0x60);
        QString::number(long, int)(&var_40h, result, 10);
        operator+(char const*, QString const&)((int64_t)&var_38h, (int64_t)"BFS[", (int64_t)&var_40h);
        operator+(QString const&, char const*)((int64_t)&var_30h, (int64_t)&var_38h, 0x956c);
        QLineEdit::setText(QString const&)(uVar1, &var_30h);
        QString::~QString()((int64_t)&var_30h);
        QString::~QString()((int64_t)&var_38h);
        QString::~QString()((int64_t)&var_40h);
    }
    if (canary != *(int64_t *)(in_FS_OFFSET + 0x28)) {
    // WARNING: Subroutine does not return
        __stack_chk_fail();
    }
    return;
}
```
