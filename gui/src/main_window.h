#pragma once

#include <QMainWindow>
#include <QProcess>

class QPlainTextEdit;
class QLineEdit;
class QPushButton;
class QLabel;
class QTabWidget;
class QTimer;
class QSpinBox;
class QCheckBox;

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    MainWindow();

private slots:
    void readStdout();
    void readStderr();
    void readMetricsStdout();
    void readMetricsStderr();
    void processFinished(int exitCode, QProcess::ExitStatus status);
    void refreshMetrics();
    void metricsFinished(int exitCode, QProcess::ExitStatus status);
    void applySettings();

private:
    void buildUi();
    void setBusy(bool busy);
    void runCli(const QStringList &args, QPlainTextEdit *output = nullptr);
    void appendOutput(const QString &text);
    void parseMetricsOutput(const QString &text);
    QString cliPath() const;

    QTabWidget *tabs_ = nullptr;
    QPlainTextEdit *console_ = nullptr;
    QPlainTextEdit *newsOutput_ = nullptr;
    QPlainTextEdit *activeOutput_ = nullptr;
    QLabel *status_ = nullptr;

    QLineEdit *pkgInput_ = nullptr;
    QLineEdit *serviceInput_ = nullptr;
    QLineEdit *userInput_ = nullptr;
    QLineEdit *snapshotInput_ = nullptr;
    QLineEdit *ifaceInput_ = nullptr;
    QLineEdit *logLinesInput_ = nullptr;
    QLineEdit *searchInput_ = nullptr;

    QLabel *ramLabel_ = nullptr;
    QLabel *cpuTempLabel_ = nullptr;
    QLabel *cpuUsageLabel_ = nullptr;
    QLabel *gpuLabel_ = nullptr;
    QLabel *gpuTempLabel_ = nullptr;
    QLabel *gpuUsageLabel_ = nullptr;
    QLabel *batteryLabel_ = nullptr;
    QLabel *batteryHealthLabel_ = nullptr;
    QLabel *loadLabel_ = nullptr;
    QLabel *memUsageLabel_ = nullptr;
    QLabel *netStatusLabel_ = nullptr;
    QLabel *netIpLabel_ = nullptr;
    QLabel *netTrafficLabel_ = nullptr;
    QLabel *orphansLabel_ = nullptr;
    QLabel *uptimeLabel_ = nullptr;
    QPlainTextEdit *processList_ = nullptr;

    QLineEdit *cliPathInput_ = nullptr;
    QSpinBox *refreshIntervalInput_ = nullptr;
    QCheckBox *autoRefreshCheck_ = nullptr;

    QProcess *proc_ = nullptr;
    QProcess *metricsProc_ = nullptr;
    QTimer *refreshTimer_ = nullptr;
    QString metricsStdout_;
};
