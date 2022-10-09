# QT_01

decompiled with cutter

found flag in dbg.Notepad::save()

```
// WARNING: Variable defined which should be unmapped: var_8h
// WARNING: [rz-ghidra] Type QFile of variable file has size 0
// WARNING: [rz-ghidra] Type QTextStream of variable out has size 0
// WARNING: [rz-ghidra] Detected overlap for variable var_5ch
// WARNING: [rz-ghidra] Detected overlap for variable f2
// WARNING: [rz-ghidra] Detected overlap for variable var_ch
// WARNING: [rz-ghidra] Detected overlap for variable flags

void Notepad::save()(int64_t arg1)
{
    char cVar1;
    undefined4 uVar2;
    int64_t in_FS_OFFSET;
    undefined8 uVar3;
    Notepad *this;
    undefined4 var_5ch;
    QString fileName;
    QString text;
    int64_t var_48h;
    undefined auStack72 [16];
    undefined auStack56 [24];
    int64_t canary;
    int64_t var_8h;
    
    canary = *(int64_t *)(in_FS_OFFSET + 0x28);
    QString::QString()((int64_t)&fileName);
    cVar1 = QString::isEmpty() const(arg1 + 0x38);
    if (cVar1 == '\0') {
        QString::operator=(QString const&)(&fileName, arg1 + 0x38);
    } else {
        QFlags<QFileDialog::Option>::QFlags(int QFlags<QFileDialog::Option>::Private::*)((int64_t)&var_5ch, -1);
        QString::QString()((int64_t)auStack56);
        QString::QString()((int64_t)auStack72);
        uVar3 = 0x5d4a;
        QString::QString(char const*)((int64_t)&text, "Save");
        QFileDialog::getSaveFileName(QWidget*, QString const&, QString const&, QString const&, QString*, QFlags<QFileDialog::Option>)
                  (&var_48h, arg1, &text, auStack72, auStack56, 0, var_5ch, uVar3);
        QString::operator=(QString&&)((int64_t)&fileName, (int64_t)&var_48h);
        QString::~QString()((int64_t)&var_48h);
        QString::~QString()((int64_t)&text);
        QString::~QString()((int64_t)auStack72);
        QString::~QString()((int64_t)auStack56);
        QString::operator=(QString const&)(arg1 + 0x38, &fileName);
    }
    QFile::QFile(QString const&)(auStack72, &fileName);
    uVar2 = operator|(QIODevice::OpenModeFlag, QIODevice::OpenModeFlag)(2, 0x10);
    cVar1 = QFile::open(QFlags<QIODevice::OpenModeFlag>)(auStack72, uVar2);
    if (cVar1 == '\x01') {
        QWidget::setWindowTitle(QString const&)(arg1, &fileName);
        QTextStream::QTextStream(QIODevice*)(auStack56, auStack72);
        QTextEdit::toPlainText() const(&text, *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90));
        QTextStream::operator<<(QString const&)(auStack56, &text);
        QFileDevice::close()(auStack72);
        QTextEdit::toPlainText() const(&var_48h, *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90));
        cVar1 = QString::operator==(char const*) const((int64_t)&var_48h, (int64_t)"hack the world !");
        QString::~QString()((int64_t)&var_48h);
        if (cVar1 != '\0') {
            uVar3 = *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90);
            QString::QString(char const*)((int64_t)&var_48h, "BFS[youpwndnotepad]"); <---- HERE
            QTextEdit::setText(QString const&)(uVar3, &var_48h);
            QString::~QString()((int64_t)&var_48h);
        }
        QString::~QString()((int64_t)&text);
        QTextStream::~QTextStream()(auStack56);
    } else {
        QFlags<QMessageBox::StandardButton>::QFlags(QMessageBox::StandardButton)((int64_t)&var_5ch, 0x400);
        QIODevice::errorString() const(&text, auStack72);
        operator+(char const*, QString const&)((int64_t)&var_48h, "Cannot save file: ", (int64_t)&text);
        QString::QString(char const*)((int64_t)auStack56, "Warning");
        QMessageBox::warning(QWidget*, QString const&, QString const&, QFlags<QMessageBox::StandardButton>, QMessageBox::StandardButton)
                  (arg1, auStack56, &var_48h, var_5ch, 0);
        QString::~QString()((int64_t)auStack56);
        QString::~QString()((int64_t)&var_48h);
        QString::~QString()((int64_t)&text);
    }
    QFile::~QFile()(auStack72);
    QString::~QString()((int64_t)&fileName);
    if (canary != *(int64_t *)(in_FS_OFFSET + 0x28)) {
    // WARNING: Subroutine does not return
        __stack_chk_fail();
    }
    return;
}
```
