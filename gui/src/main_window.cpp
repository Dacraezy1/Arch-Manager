#include "main_window.h"

#include <QCheckBox>
#include <QFormLayout>
#include <QGridLayout>
#include <QGroupBox>
#include <QHBoxLayout>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QProcess>
#include <QPushButton>
#include <QSpinBox>
#include <QTabWidget>
#include <QTimer>
#include <QVBoxLayout>
#include <QWidget>

MainWindow::MainWindow() {
    buildUi();
}

void MainWindow::buildUi() {
    setWindowTitle("Arch Manager");
    setMinimumSize(1200, 820);

    auto *central = new QWidget(this);
    auto *root = new QVBoxLayout(central);
    root->setContentsMargins(16, 16, 16, 16);
    root->setSpacing(12);

    tabs_ = new QTabWidget(central);

    // --- Dashboard Tab ---
    auto *dashboard = new QWidget(tabs_);
    auto *dashLayout = new QVBoxLayout(dashboard);

    auto *metricsBox = new QGroupBox("Live System Metrics", dashboard);
    auto *metricsLayout = new QGridLayout(metricsBox);

    ramLabel_ = new QLabel("RAM: —", metricsBox);
    cpuTempLabel_ = new QLabel("CPU Temp: —", metricsBox);
    cpuUsageLabel_ = new QLabel("CPU Usage: —", metricsBox);
    gpuLabel_ = new QLabel("GPU: —", metricsBox);
    gpuTempLabel_ = new QLabel("GPU Temp: —", metricsBox);
    gpuUsageLabel_ = new QLabel("GPU Usage: —", metricsBox);
    batteryLabel_ = new QLabel("Battery: —", metricsBox);
    batteryHealthLabel_ = new QLabel("Battery Health: —", metricsBox);
    loadLabel_ = new QLabel("Load: —", metricsBox);
    memUsageLabel_ = new QLabel("Memory Usage: —", metricsBox);
    netStatusLabel_ = new QLabel("Network: —", metricsBox);
    netIpLabel_ = new QLabel("IP: —", metricsBox);
    netTrafficLabel_ = new QLabel("Traffic: —", metricsBox);
    orphansLabel_ = new QLabel("Orphans: —", metricsBox);
    uptimeLabel_ = new QLabel("Uptime: —", metricsBox);

    metricsLayout->addWidget(ramLabel_, 0, 0);
    metricsLayout->addWidget(cpuTempLabel_, 0, 1);
    metricsLayout->addWidget(cpuUsageLabel_, 1, 0);
    metricsLayout->addWidget(gpuLabel_, 1, 1);
    metricsLayout->addWidget(gpuTempLabel_, 2, 0);
    metricsLayout->addWidget(gpuUsageLabel_, 2, 1);
    metricsLayout->addWidget(batteryLabel_, 3, 0);
    metricsLayout->addWidget(batteryHealthLabel_, 3, 1);
    metricsLayout->addWidget(loadLabel_, 4, 0);
    metricsLayout->addWidget(memUsageLabel_, 4, 1);
    metricsLayout->addWidget(netStatusLabel_, 5, 0);
    metricsLayout->addWidget(netIpLabel_, 5, 1);
    metricsLayout->addWidget(netTrafficLabel_, 6, 0);
    metricsLayout->addWidget(orphansLabel_, 6, 1);
    metricsLayout->addWidget(uptimeLabel_, 7, 0);

    auto *refreshMetricsBtn = new QPushButton("Refresh Metrics", metricsBox);
    metricsLayout->addWidget(refreshMetricsBtn, 8, 0, 1, 2);

    auto *processBox = new QGroupBox("Top Processes", dashboard);
    auto *processLayout = new QVBoxLayout(processBox);
    processList_ = new QPlainTextEdit(processBox);
    processList_->setReadOnly(true);
    processList_->setMinimumHeight(180);
    processLayout->addWidget(processList_);

    dashLayout->addWidget(metricsBox);
    dashLayout->addWidget(processBox);
    dashLayout->addStretch(1);
    tabs_->addTab(dashboard, "Dashboard");

    // --- Packages Tab ---
    auto *packages = new QWidget(tabs_);
    auto *pkgLayout = new QVBoxLayout(packages);

    auto *pkgBox = new QGroupBox("Package Management", packages);
    auto *pkgForm = new QFormLayout(pkgBox);
    pkgInput_ = new QLineEdit(pkgBox);
    searchInput_ = new QLineEdit(pkgBox);
    pkgForm->addRow("Package name", pkgInput_);
    pkgForm->addRow("Search query", searchInput_);

    auto *pkgButtons = new QWidget(pkgBox);
    auto *pkgButtonsLayout = new QGridLayout(pkgButtons);
    auto *pkgInstallBtn = new QPushButton("Install", pkgButtons);
    auto *pkgRemoveBtn = new QPushButton("Remove", pkgButtons);
    auto *pkgSearchBtn = new QPushButton("Search", pkgButtons);
    auto *pkgInfoBtn = new QPushButton("Info", pkgButtons);
    auto *pkgUpdateBtn = new QPushButton("Sync DB", pkgButtons);
    auto *pkgUpgradeBtn = new QPushButton("Upgrade", pkgButtons);
    auto *pkgCleanBtn = new QPushButton("Clean Cache", pkgButtons);

    pkgButtonsLayout->addWidget(pkgInstallBtn, 0, 0);
    pkgButtonsLayout->addWidget(pkgRemoveBtn, 0, 1);
    pkgButtonsLayout->addWidget(pkgSearchBtn, 1, 0);
    pkgButtonsLayout->addWidget(pkgInfoBtn, 1, 1);
    pkgButtonsLayout->addWidget(pkgUpdateBtn, 2, 0);
    pkgButtonsLayout->addWidget(pkgUpgradeBtn, 2, 1);
    pkgButtonsLayout->addWidget(pkgCleanBtn, 3, 0, 1, 2);

    pkgLayout->addWidget(pkgBox);
    pkgLayout->addWidget(pkgButtons);
    pkgLayout->addStretch(1);

    tabs_->addTab(packages, "Packages");

    // --- Settings Tab ---
    auto *settings = new QWidget(tabs_);
    auto *settingsLayout = new QVBoxLayout(settings);
    auto *settingsBox = new QGroupBox("Preferences", settings);
    auto *settingsForm = new QFormLayout(settingsBox);

    cliPathInput_ = new QLineEdit(settingsBox);
    cliPathInput_->setPlaceholderText("/usr/bin/arch-manager");
    refreshIntervalInput_ = new QSpinBox(settingsBox);
    refreshIntervalInput_->setRange(5, 600);
    refreshIntervalInput_->setValue(30);
    autoRefreshCheck_ = new QCheckBox("Auto refresh metrics", settingsBox);
    autoRefreshCheck_->setChecked(true);

    settingsForm->addRow("CLI Path", cliPathInput_);
    settingsForm->addRow("Refresh interval (sec)", refreshIntervalInput_);
    settingsForm->addRow("", autoRefreshCheck_);

    auto *applyButton = new QPushButton("Apply Settings", settingsBox);
    settingsLayout->addWidget(settingsBox);
    settingsLayout->addWidget(applyButton);
    settingsLayout->addStretch(1);
    tabs_->addTab(settings, "Settings");

    // --- Console Section ---
    auto *consoleBox = new QGroupBox("Console", central);
    auto *consoleLayout = new QVBoxLayout(consoleBox);
    console_ = new QPlainTextEdit(consoleBox);
    console_->setReadOnly(true);
    console_->setMinimumHeight(180);

    auto *consoleActions = new QHBoxLayout();
    auto *clearBtn = new QPushButton("Clear", consoleBox);
    status_ = new QLabel("Idle", consoleBox);
    consoleActions->addWidget(clearBtn);
    consoleActions->addStretch(1);
    consoleActions->addWidget(status_);

    consoleLayout->addWidget(console_);
    consoleLayout->addLayout(consoleActions);

    root->addWidget(tabs_);
    root->addWidget(consoleBox);
    setCentralWidget(central);

    // --- Initialize QProcesses and QTimer ---
    proc_ = new QProcess(this);
    metricsProc_ = new QProcess(this);
    refreshTimer_ = new QTimer(this);

    // --- Connect signals ---
    connect(proc_, &QProcess::readyReadStandardOutput, this, &MainWindow::readStdout);
    connect(proc_, &QProcess::readyReadStandardError, this, &MainWindow::readStderr);
    connect(proc_, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &MainWindow::processFinished);

    connect(metricsProc_, &QProcess::readyReadStandardOutput, this, &MainWindow::readMetricsStdout);
    connect(metricsProc_, &QProcess::readyReadStandardError, this, &MainWindow::readMetricsStderr);
    connect(metricsProc_, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &MainWindow::metricsFinished);

    connect(refreshMetricsBtn, &QPushButton::clicked, this, &MainWindow::refreshMetrics);
    connect(clearBtn, &QPushButton::clicked, console_, &QPlainTextEdit::clear);
    connect(applyButton, &QPushButton::clicked, this, &MainWindow::applySettings);

    // --- Example: Package Buttons ---
    connect(pkgInstallBtn, &QPushButton::clicked, this, [this]() {
        const QString pkg = pkgInput_->text().trimmed();
        if (!pkg.isEmpty()) runCli({"pacman", "install", pkg});
    });
    connect(pkgRemoveBtn, &QPushButton::clicked, this, [this]() {
        const QString pkg = pkgInput_->text().trimmed();
        if (!pkg.isEmpty()) runCli({"pacman", "remove", pkg});
    });
    connect(pkgSearchBtn, &QPushButton::clicked, this, [this]() {
        const QString query = searchInput_->text().trimmed();
        if (!query.isEmpty()) runCli({"pacman", "search", query});
    });
    connect(pkgInfoBtn, &QPushButton::clicked, this, [this]() {
        const QString pkg = pkgInput_->text().trimmed();
        if (!pkg.isEmpty()) runCli({"pacman", "info", pkg});
    });
    connect(pkgUpdateBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "update"});
    });
    connect(pkgUpgradeBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "upgrade"});
    });
    connect(pkgCleanBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "clean"});
    });

    // --- Auto-refresh metrics ---
    connect(refreshTimer_, &QTimer::timeout, this, &MainWindow::refreshMetrics);
    refreshTimer_->start(refreshIntervalInput_->value() * 1000);

    // --- Stylesheet ---
    setStyleSheet(R"(
        QMainWindow { background: #0f172a; }
        QTabWidget::pane { border: 0; }
        QTabBar::tab { background: #111827; color: #cbd5f5; padding: 10px 16px; border-radius: 8px; margin-right: 6px; }
        QTabBar::tab:selected { background: #2563eb; color: #ffffff; }
        QGroupBox { border: 1px solid #334155; border-radius: 12px; margin-top: 12px; background: #0b1220; color: #e2e8f0; }
        QGroupBox::title { subcontrol-origin: margin; subcontrol-position: top left; padding: 6px 10px; background: #1f2937; border-radius: 8px; margin-left: 12px; }
        QLabel { color: #e2e8f0; }
        QLineEdit, QPlainTextEdit { background: #0f172a; border: 1px solid #334155; color: #e2e8f0; padding: 6px; border-radius: 8px; }
        QPushButton { background: #1d4ed8; color: white; border: none; padding: 8px 14px; border-radius: 8px; }
        QPushButton:hover { background: #2563eb; }
        QPushButton:disabled { background: #334155; }
    )");

    refreshMetrics(); // initial metrics fetch
}

// --- Core CLI runner ---
void MainWindow::runCli(const QStringList &args, QPlainTextEdit *output) {
    if (proc_->state() != QProcess::NotRunning) {
        appendOutput("Process already running. Please wait.\n");
        return;
    }

    QStringList actual = args;
    actual.removeAll("");

    activeOutput_ = output ? output : console_;
    if (activeOutput_) activeOutput_->clear();

    if (actual.isEmpty()) {
        appendOutput("No command provided.\n");
        return;
    }

    const QString program = cliPath();
    if (program.isEmpty()) {
        appendOutput("CLI path not set.\n");
        return;
    }

    status_->setText("Running...");
    setBusy(true);
    proc_->start(program, actual);
}

// --- Output handling ---
void MainWindow::readStdout() {
    if (!activeOutput_) return;
    activeOutput_->appendPlainText(QString::fromUtf8(proc_->readAllStandardOutput()));
}

void MainWindow::readStderr() {
    if (!activeOutput_) return;
    activeOutput_->appendPlainText(QString::fromUtf8(proc_->readAllStandardError()));
}

void MainWindow::readMetricsStdout() {
    metricsStdout_.append(QString::fromUtf8(metricsProc_->readAllStandardOutput()));
}

void MainWindow::readMetricsStderr() {
    console_->appendPlainText(QString::fromUtf8(metricsProc_->readAllStandardError()));
}

// --- Process finished ---
void MainWindow::processFinished(int exitCode, QProcess::ExitStatus status) {
    Q_UNUSED(status)
    status_->setText(QString("Finished (exit %1)").arg(exitCode));
    setBusy(false);
    activeOutput_ = console_;
}

// --- Metrics ---
void MainWindow::refreshMetrics() {
    if (metricsProc_->state() != QProcess::NotRunning) return;

    activeOutput_ = console_; // or dedicated metrics widget
    metricsStdout_.clear();
    metricsProc_->start(cliPath(), {"metrics", "all"});
}

void MainWindow::metricsFinished(int exitCode, QProcess::ExitStatus status) {
    Q_UNUSED(exitCode)
    Q_UNUSED(status)
    parseMetricsOutput(metricsStdout_);
}

// --- Metrics parsing ---
void MainWindow::parseMetricsOutput(const QString &text) {
    const auto lines = text.split('\n', Qt::SkipEmptyParts);
    for (const auto &line : lines) {
        const int idx = line.indexOf(":");
        if (idx <= 0) continue;
        const QString key = line.left(idx).trimmed();
        const QString value = line.mid(idx + 1).trimmed();

        if (key == "RAM") ramLabel_->setText("RAM: " + value);
        else if (key == "CPU_TEMP") cpuTempLabel_->setText("CPU Temp: " + value);
        else if (key == "CPU_USAGE") cpuUsageLabel_->setText("CPU Usage: " + value);
        else if (key == "GPU") gpuLabel_->setText("GPU: " + value);
        else if (key == "GPU_TEMP") gpuTempLabel_->setText("GPU Temp: " + value);
        else if (key == "GPU_USAGE") gpuUsageLabel_->setText("GPU Usage: " + value);
        else if (key == "BATTERY") batteryLabel_->setText("Battery: " + value);
        else if (key == "BATTERY_HEALTH") batteryHealthLabel_->setText("Battery Health: " + value);
        else if (key == "LOAD") loadLabel_->setText("Load: " + value);
        else if (key == "MEM_USAGE") memUsageLabel_->setText("Memory Usage: " + value);
        else if (key == "NET_STATUS") netStatusLabel_->setText("Network: " + value);
        else if (key == "NET_IP") netIpLabel_->setText("IP: " + value);
        else if (key == "NET_TRAFFIC") netTrafficLabel_->setText("Traffic: " + value);
        else if (key == "ORPHANS") orphansLabel_->setText("Orphans: " + value);
        else if (key == "UPTIME") uptimeLabel_->setText("Uptime: " + value);
        else if (key == "PROCESSES") processList_->setPlainText(value.replace(" | ", "\n"));
    }
}

// --- Settings ---
void MainWindow::applySettings() {
    const int intervalMs = refreshIntervalInput_->value() * 1000;
    refreshTimer_->stop();
    if (autoRefreshCheck_->isChecked()) {
        refreshTimer_->start(intervalMs);
    }
    status_->setText("Settings applied");
}

QString MainWindow::cliPath() const {
    const QString path = cliPathInput_->text().trimmed();
    return path.isEmpty() ? "/usr/bin/arch-manager" : path;
}

void MainWindow::appendOutput(const QString &text) {
    if (!activeOutput_) return;
    activeOutput_->appendPlainText(text);
}

void MainWindow::setBusy(bool busy) {
    tabs_->setEnabled(!busy);
}
