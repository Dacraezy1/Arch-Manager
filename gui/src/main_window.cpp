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

    auto *dashboard = new QWidget(tabs_);
    auto *dashLayout = new QVBoxLayout(dashboard);

    auto *metricsBox = new QGroupBox("Live System Metrics", dashboard);
    auto *metricsLayout = new QGridLayout(metricsBox);

    ramLabel_ = new QLabel("RAM: —", metricsBox);
    cpuTempLabel_ = new QLabel("CPU Temp: —", metricsBox);
    gpuLabel_ = new QLabel("GPU: —", metricsBox);
    batteryLabel_ = new QLabel("Battery: —", metricsBox);
    loadLabel_ = new QLabel("Load: —", metricsBox);
    uptimeLabel_ = new QLabel("Uptime: —", metricsBox);

    metricsLayout->addWidget(ramLabel_, 0, 0);
    metricsLayout->addWidget(cpuTempLabel_, 0, 1);
    metricsLayout->addWidget(gpuLabel_, 1, 0);
    metricsLayout->addWidget(batteryLabel_, 1, 1);
    metricsLayout->addWidget(loadLabel_, 2, 0);
    metricsLayout->addWidget(uptimeLabel_, 2, 1);

    auto *refreshMetricsBtn = new QPushButton("Refresh Metrics", metricsBox);
    metricsLayout->addWidget(refreshMetricsBtn, 3, 0, 1, 2);

    auto *healthBox = new QGroupBox("Health", dashboard);
    auto *healthLayout = new QHBoxLayout(healthBox);
    auto *healthSummaryBtn = new QPushButton("Summary", healthBox);
    auto *healthFullBtn = new QPushButton("Full Report", healthBox);
    healthLayout->addWidget(healthSummaryBtn);
    healthLayout->addWidget(healthFullBtn);

    dashLayout->addWidget(metricsBox);
    dashLayout->addWidget(healthBox);
    dashLayout->addStretch(1);

    tabs_->addTab(dashboard, "Dashboard");

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

    auto *updatesBox = new QGroupBox("Updates", packages);
    auto *updatesLayout = new QHBoxLayout(updatesBox);
    auto *updatesCheckBtn = new QPushButton("Check", updatesBox);
    auto *updatesListBtn = new QPushButton("List", updatesBox);
    updatesLayout->addWidget(updatesCheckBtn);
    updatesLayout->addWidget(updatesListBtn);
    pkgLayout->addWidget(updatesBox);
    pkgLayout->addStretch(1);

    tabs_->addTab(packages, "Packages");

    auto *services = new QWidget(tabs_);
    auto *svcLayout = new QVBoxLayout(services);

    auto *svcBox = new QGroupBox("Systemd Services", services);
    auto *svcForm = new QFormLayout(svcBox);
    serviceInput_ = new QLineEdit(svcBox);
    svcForm->addRow("Service name", serviceInput_);

    auto *svcButtons = new QWidget(svcBox);
    auto *svcButtonsLayout = new QGridLayout(svcButtons);
    auto *svcStatusBtn = new QPushButton("Status", svcButtons);
    auto *svcStartBtn = new QPushButton("Start", svcButtons);
    auto *svcStopBtn = new QPushButton("Stop", svcButtons);
    auto *svcRestartBtn = new QPushButton("Restart", svcButtons);
    auto *svcEnableBtn = new QPushButton("Enable", svcButtons);
    auto *svcDisableBtn = new QPushButton("Disable", svcButtons);
    auto *svcListBtn = new QPushButton("List Services", svcButtons);

    svcButtonsLayout->addWidget(svcStatusBtn, 0, 0);
    svcButtonsLayout->addWidget(svcStartBtn, 0, 1);
    svcButtonsLayout->addWidget(svcStopBtn, 1, 0);
    svcButtonsLayout->addWidget(svcRestartBtn, 1, 1);
    svcButtonsLayout->addWidget(svcEnableBtn, 2, 0);
    svcButtonsLayout->addWidget(svcDisableBtn, 2, 1);
    svcButtonsLayout->addWidget(svcListBtn, 3, 0, 1, 2);

    svcLayout->addWidget(svcBox);
    svcLayout->addWidget(svcButtons);
    svcLayout->addStretch(1);

    tabs_->addTab(services, "Services");

    auto *system = new QWidget(tabs_);
    auto *sysLayout = new QVBoxLayout(system);

    auto *hardwareBox = new QGroupBox("Hardware", system);
    auto *hardwareLayout = new QGridLayout(hardwareBox);
    auto *hwCpuBtn = new QPushButton("CPU", hardwareBox);
    auto *hwMemBtn = new QPushButton("Memory", hardwareBox);
    auto *hwGpuBtn = new QPushButton("GPU", hardwareBox);
    auto *hwUsbBtn = new QPushButton("USB", hardwareBox);
    auto *hwPciBtn = new QPushButton("PCI", hardwareBox);

    hardwareLayout->addWidget(hwCpuBtn, 0, 0);
    hardwareLayout->addWidget(hwMemBtn, 0, 1);
    hardwareLayout->addWidget(hwGpuBtn, 1, 0);
    hardwareLayout->addWidget(hwUsbBtn, 1, 1);
    hardwareLayout->addWidget(hwPciBtn, 2, 0, 1, 2);

    auto *systemBox = new QGroupBox("System Info", system);
    auto *systemLayout = new QHBoxLayout(systemBox);
    auto *diskBtn = new QPushButton("Disk Usage", systemBox);
    auto *mountsBtn = new QPushButton("Mounts", systemBox);
    auto *memBtn = new QPushButton("Memory", systemBox);
    auto *kernelBtn = new QPushButton("Kernel", systemBox);

    systemLayout->addWidget(diskBtn);
    systemLayout->addWidget(mountsBtn);
    systemLayout->addWidget(memBtn);
    systemLayout->addWidget(kernelBtn);

    sysLayout->addWidget(hardwareBox);
    sysLayout->addWidget(systemBox);
    sysLayout->addStretch(1);

    tabs_->addTab(system, "System");

    auto *logs = new QWidget(tabs_);
    auto *logsLayout = new QVBoxLayout(logs);

    auto *logsBox = new QGroupBox("Journal Logs", logs);
    auto *logsForm = new QFormLayout(logsBox);
    logLinesInput_ = new QLineEdit(logsBox);
    logLinesInput_->setPlaceholderText("200");
    logsForm->addRow("Tail lines", logLinesInput_);

    auto *logsButtons = new QWidget(logsBox);
    auto *logsButtonsLayout = new QGridLayout(logsButtons);
    auto *logsTailBtn = new QPushButton("Tail", logsButtons);
    auto *logsBootBtn = new QPushButton("Boot", logsButtons);
    auto *logsServiceBtn = new QPushButton("Service", logsButtons);

    logsButtonsLayout->addWidget(logsTailBtn, 0, 0);
    logsButtonsLayout->addWidget(logsBootBtn, 0, 1);
    logsButtonsLayout->addWidget(logsServiceBtn, 1, 0, 1, 2);

    logsLayout->addWidget(logsBox);
    logsLayout->addWidget(logsButtons);
    logsLayout->addStretch(1);

    tabs_->addTab(logs, "Logs");

    auto *network = new QWidget(tabs_);
    auto *netLayout = new QVBoxLayout(network);

    auto *netBox = new QGroupBox("Network", network);
    auto *netForm = new QFormLayout(netBox);
    ifaceInput_ = new QLineEdit(netBox);
    netForm->addRow("Interface", ifaceInput_);

    auto *netButtons = new QWidget(netBox);
    auto *netButtonsLayout = new QGridLayout(netButtons);
    auto *netListBtn = new QPushButton("List Devices", netButtons);
    auto *netUpBtn = new QPushButton("Connect", netButtons);
    auto *netDownBtn = new QPushButton("Disconnect", netButtons);
    auto *netWifiBtn = new QPushButton("WiFi Scan", netButtons);

    netButtonsLayout->addWidget(netListBtn, 0, 0);
    netButtonsLayout->addWidget(netWifiBtn, 0, 1);
    netButtonsLayout->addWidget(netUpBtn, 1, 0);
    netButtonsLayout->addWidget(netDownBtn, 1, 1);

    netLayout->addWidget(netBox);
    netLayout->addWidget(netButtons);
    netLayout->addStretch(1);

    tabs_->addTab(network, "Network");

    auto *power = new QWidget(tabs_);
    auto *powerLayout = new QVBoxLayout(power);
    auto *powerBox = new QGroupBox("Power & Battery", power);
    auto *powerButtons = new QHBoxLayout(powerBox);
    auto *batteryBtn = new QPushButton("Battery Health", powerBox);
    auto *cpuTempBtn = new QPushButton("CPU Temp", powerBox);
    auto *gpuUsageBtn = new QPushButton("GPU Usage", powerBox);

    powerButtons->addWidget(batteryBtn);
    powerButtons->addWidget(cpuTempBtn);
    powerButtons->addWidget(gpuUsageBtn);

    powerLayout->addWidget(powerBox);
    powerLayout->addStretch(1);

    tabs_->addTab(power, "Power");

    auto *maintenance = new QWidget(tabs_);
    auto *maintLayout = new QVBoxLayout(maintenance);
    auto *maintBox = new QGroupBox("Maintenance", maintenance);
    auto *maintButtons = new QGridLayout(maintBox);
    auto *failedServicesBtn = new QPushButton("Failed Services", maintBox);
    auto *cleanCacheBtn = new QPushButton("Clean Cache", maintBox);
    auto *newsBtn = new QPushButton("Arch News", maintBox);

    maintButtons->addWidget(failedServicesBtn, 0, 0);
    maintButtons->addWidget(cleanCacheBtn, 0, 1);
    maintButtons->addWidget(newsBtn, 1, 0, 1, 2);

    maintLayout->addWidget(maintBox);
    maintLayout->addStretch(1);

    tabs_->addTab(maintenance, "Maintenance");

    auto *users = new QWidget(tabs_);
    auto *userLayout = new QVBoxLayout(users);
    auto *userBox = new QGroupBox("Users", users);
    auto *userForm = new QFormLayout(userBox);
    userInput_ = new QLineEdit(userBox);
    userForm->addRow("Username", userInput_);

    auto *userButtons = new QWidget(userBox);
    auto *userButtonsLayout = new QGridLayout(userButtons);
    auto *userAddBtn = new QPushButton("Add", userButtons);
    auto *userDelBtn = new QPushButton("Delete", userButtons);
    auto *userPassBtn = new QPushButton("Password", userButtons);
    auto *userListBtn = new QPushButton("List", userButtons);

    userButtonsLayout->addWidget(userAddBtn, 0, 0);
    userButtonsLayout->addWidget(userDelBtn, 0, 1);
    userButtonsLayout->addWidget(userPassBtn, 1, 0);
    userButtonsLayout->addWidget(userListBtn, 1, 1);

    userLayout->addWidget(userBox);
    userLayout->addWidget(userButtons);
    userLayout->addStretch(1);

    tabs_->addTab(users, "Users");

    auto *snapshots = new QWidget(tabs_);
    auto *snapLayout = new QVBoxLayout(snapshots);
    auto *snapBox = new QGroupBox("Btrfs Snapshots", snapshots);
    auto *snapForm = new QFormLayout(snapBox);
    snapshotInput_ = new QLineEdit(snapBox);
    snapForm->addRow("Snapshot name", snapshotInput_);

    auto *snapButtons = new QWidget(snapBox);
    auto *snapButtonsLayout = new QGridLayout(snapButtons);
    auto *snapListBtn = new QPushButton("List", snapButtons);
    auto *snapCreateBtn = new QPushButton("Create", snapButtons);
    auto *snapDeleteBtn = new QPushButton("Delete", snapButtons);

    snapButtonsLayout->addWidget(snapListBtn, 0, 0);
    snapButtonsLayout->addWidget(snapCreateBtn, 0, 1);
    snapButtonsLayout->addWidget(snapDeleteBtn, 1, 0, 1, 2);

    snapLayout->addWidget(snapBox);
    snapLayout->addWidget(snapButtons);
    snapLayout->addStretch(1);

    tabs_->addTab(snapshots, "Snapshots");

    auto *news = new QWidget(tabs_);
    auto *newsLayout = new QVBoxLayout(news);
    auto *newsBox = new QGroupBox("Arch News", news);
    auto *newsBoxLayout = new QVBoxLayout(newsBox);
    auto *newsRefreshBtn = new QPushButton("Refresh News", newsBox);
    newsOutput_ = new QPlainTextEdit(newsBox);
    newsOutput_->setReadOnly(true);

    newsBoxLayout->addWidget(newsRefreshBtn);
    newsBoxLayout->addWidget(newsOutput_);
    newsLayout->addWidget(newsBox);
    newsLayout->addStretch(1);

    tabs_->addTab(news, "News");

    auto *settings = new QWidget(tabs_);
    auto *settingsLayout = new QVBoxLayout(settings);
    auto *settingsBox = new QGroupBox("Settings", settings);
    auto *settingsForm = new QFormLayout(settingsBox);

    cliPathInput_ = new QLineEdit(settingsBox);
    cliPathInput_->setText("arch-manager");
    refreshIntervalInput_ = new QSpinBox(settingsBox);
    refreshIntervalInput_->setRange(5, 3600);
    refreshIntervalInput_->setValue(30);
    autoRefreshCheck_ = new QCheckBox("Auto refresh metrics", settingsBox);
    autoRefreshCheck_->setChecked(true);

    settingsForm->addRow("CLI path", cliPathInput_);
    settingsForm->addRow("Refresh interval (sec)", refreshIntervalInput_);
    settingsForm->addRow("", autoRefreshCheck_);

    auto *applyBtn = new QPushButton("Apply", settingsBox);
    settingsForm->addRow("", applyBtn);

    settingsLayout->addWidget(settingsBox);
    settingsLayout->addStretch(1);

    tabs_->addTab(settings, "Settings");

    root->addWidget(tabs_);

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

    root->addWidget(consoleBox);
    setCentralWidget(central);

    proc_ = new QProcess(this);
    metricsProc_ = new QProcess(this);
    refreshTimer_ = new QTimer(this);

    connect(proc_, &QProcess::readyReadStandardOutput, this, &MainWindow::readStdout);
    connect(proc_, &QProcess::readyReadStandardError, this, &MainWindow::readStderr);
    connect(proc_, &QProcess::finished, this, &MainWindow::processFinished);

    connect(metricsProc_, &QProcess::finished, this, &MainWindow::metricsFinished);
    connect(refreshMetricsBtn, &QPushButton::clicked, this, &MainWindow::refreshMetrics);
    connect(clearBtn, &QPushButton::clicked, console_, &QPlainTextEdit::clear);
    connect(applyBtn, &QPushButton::clicked, this, &MainWindow::applySettings);

    connect(healthSummaryBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "summary"});
    });
    connect(healthFullBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "full"});
    });

    connect(pkgInstallBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "install", pkgInput_->text()});
    });
    connect(pkgRemoveBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "remove", pkgInput_->text()});
    });
    connect(pkgSearchBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "search", searchInput_->text()});
    });
    connect(pkgInfoBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "info", pkgInput_->text()});
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
    connect(updatesCheckBtn, &QPushButton::clicked, this, [this]() {
        runCli({"updates", "check"});
    });
    connect(updatesListBtn, &QPushButton::clicked, this, [this]() {
        runCli({"updates", "list"});
    });

    connect(svcStatusBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "status", serviceInput_->text()});
    });
    connect(svcStartBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "start", serviceInput_->text()});
    });
    connect(svcStopBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "stop", serviceInput_->text()});
    });
    connect(svcRestartBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "restart", serviceInput_->text()});
    });
    connect(svcEnableBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "enable", serviceInput_->text()});
    });
    connect(svcDisableBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "disable", serviceInput_->text()});
    });
    connect(svcListBtn, &QPushButton::clicked, this, [this]() {
        runCli({"systemd", "list"});
    });

    connect(hwCpuBtn, &QPushButton::clicked, this, [this]() {
        runCli({"hardware", "cpu"});
    });
    connect(hwMemBtn, &QPushButton::clicked, this, [this]() {
        runCli({"hardware", "mem"});
    });
    connect(hwGpuBtn, &QPushButton::clicked, this, [this]() {
        runCli({"hardware", "gpu"});
    });
    connect(hwUsbBtn, &QPushButton::clicked, this, [this]() {
        runCli({"hardware", "usb"});
    });
    connect(hwPciBtn, &QPushButton::clicked, this, [this]() {
        runCli({"hardware", "pci"});
    });

    connect(diskBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "disk"});
    });
    connect(mountsBtn, &QPushButton::clicked, this, [this]() {
        runCli({"disks", "mounts"});
    });
    connect(memBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "memory"});
    });
    connect(kernelBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "kernel"});
    });

    connect(logsTailBtn, &QPushButton::clicked, this, [this]() {
        const QString lines = logLinesInput_->text().isEmpty() ? "200" : logLinesInput_->text();
        runCli({"logs", "tail", lines});
    });
    connect(logsBootBtn, &QPushButton::clicked, this, [this]() {
        runCli({"logs", "boot"});
    });
    connect(logsServiceBtn, &QPushButton::clicked, this, [this]() {
        runCli({"logs", "service", serviceInput_->text()});
    });

    connect(netListBtn, &QPushButton::clicked, this, [this]() {
        runCli({"network", "list"});
    });
    connect(netWifiBtn, &QPushButton::clicked, this, [this]() {
        runCli({"network", "wifi-scan"});
    });
    connect(netUpBtn, &QPushButton::clicked, this, [this]() {
        runCli({"network", "up", ifaceInput_->text()});
    });
    connect(netDownBtn, &QPushButton::clicked, this, [this]() {
        runCli({"network", "down", ifaceInput_->text()});
    });

    connect(batteryBtn, &QPushButton::clicked, this, [this]() {
        runCli({"metrics", "battery"});
    });
    connect(cpuTempBtn, &QPushButton::clicked, this, [this]() {
        runCli({"metrics", "cpu-temp"});
    });
    connect(gpuUsageBtn, &QPushButton::clicked, this, [this]() {
        runCli({"metrics", "gpu"});
    });

    connect(failedServicesBtn, &QPushButton::clicked, this, [this]() {
        runCli({"health", "services"});
    });
    connect(cleanCacheBtn, &QPushButton::clicked, this, [this]() {
        runCli({"pacman", "clean"});
    });
    connect(newsBtn, &QPushButton::clicked, this, [this]() {
        runCli({"news", "latest"});
    });

    connect(userAddBtn, &QPushButton::clicked, this, [this]() {
        runCli({"users", "add", userInput_->text()});
    });
    connect(userDelBtn, &QPushButton::clicked, this, [this]() {
        runCli({"users", "del", userInput_->text()});
    });
    connect(userPassBtn, &QPushButton::clicked, this, [this]() {
        runCli({"users", "passwd", userInput_->text()});
    });
    connect(userListBtn, &QPushButton::clicked, this, [this]() {
        runCli({"users", "list"});
    });

    connect(snapListBtn, &QPushButton::clicked, this, [this]() {
        runCli({"snapshots", "list"});
    });
    connect(snapCreateBtn, &QPushButton::clicked, this, [this]() {
        runCli({"snapshots", "create", snapshotInput_->text()});
    });
    connect(snapDeleteBtn, &QPushButton::clicked, this, [this]() {
        runCli({"snapshots", "delete", snapshotInput_->text()});
    });

    connect(newsRefreshBtn, &QPushButton::clicked, this, [this]() {
        runCli({"news", "latest"}, newsOutput_);
    });

    connect(refreshTimer_, &QTimer::timeout, this, &MainWindow::refreshMetrics);

    setStyleSheet(R"(
        QMainWindow {
            background: qlineargradient(x1:0, y1:0, x2:1, y2:1,
                stop:0 #0f172a, stop:1 #1e293b);
        }
        QTabWidget::pane { border: 0; }
        QTabBar::tab {
            background: #111827;
            color: #cbd5f5;
            padding: 10px 16px;
            border-radius: 8px;
            margin-right: 6px;
        }
        QTabBar::tab:selected { background: #2563eb; color: #ffffff; }
        QGroupBox {
            border: 1px solid #334155;
            border-radius: 12px;
            margin-top: 12px;
            background: #0b1220;
            color: #e2e8f0;
        }
        QGroupBox::title {
            subcontrol-origin: margin;
            subcontrol-position: top left;
            padding: 6px 10px;
            background: #1f2937;
            border-radius: 8px;
            margin-left: 12px;
        }
        QLabel { color: #e2e8f0; }
        QLineEdit, QPlainTextEdit {
            background: #0f172a;
            border: 1px solid #334155;
            color: #e2e8f0;
            padding: 6px;
            border-radius: 8px;
        }
        QPushButton {
            background: #1d4ed8;
            color: white;
            border: none;
            padding: 8px 14px;
            border-radius: 8px;
        }
        QPushButton:hover { background: #2563eb; }
        QPushButton:disabled { background: #334155; }
    )");

    applySettings();
    refreshMetrics();
}

void MainWindow::runCli(const QStringList &args, QPlainTextEdit *output) {
    if (proc_->state() != QProcess::NotRunning) {
        appendOutput("Process already running. Please wait.\n");
        return;
    }

    QStringList actual = args;
    actual.removeAll("");

    activeOutput_ = output ? output : console_;
    if (activeOutput_) activeOutput_->clear();

    status_->setText("Running...");
    setBusy(true);
    proc_->start(cliPath(), actual);
}

void MainWindow::readStdout() {
    appendOutput(QString::fromUtf8(proc_->readAllStandardOutput()));
}

void MainWindow::readStderr() {
    appendOutput(QString::fromUtf8(proc_->readAllStandardError()));
}

void MainWindow::processFinished(int exitCode, QProcess::ExitStatus status) {
    Q_UNUSED(status)
    status_->setText(QString("Finished (exit %1)").arg(exitCode));
    setBusy(false);
    activeOutput_ = console_;
}

void MainWindow::appendOutput(const QString &text) {
    if (!activeOutput_) return;
    activeOutput_->appendPlainText(text);
}

void MainWindow::setBusy(bool busy) {
    tabs_->setEnabled(!busy);
}

void MainWindow::refreshMetrics() {
    if (metricsProc_->state() != QProcess::NotRunning) {
        return;
    }
    metricsProc_->start(cliPath(), {"metrics", "all"});
}

void MainWindow::metricsFinished(int exitCode, QProcess::ExitStatus status) {
    Q_UNUSED(exitCode)
    Q_UNUSED(status)
    const QString text = QString::fromUtf8(metricsProc_->readAllStandardOutput());
    parseMetricsOutput(text);
}

void MainWindow::parseMetricsOutput(const QString &text) {
    const auto lines = text.split('\n', Qt::SkipEmptyParts);
    for (const auto &line : lines) {
        const int idx = line.indexOf(":");
        if (idx <= 0) continue;
        const QString key = line.left(idx).trimmed();
        const QString value = line.mid(idx + 1).trimmed();

        if (key == "RAM") ramLabel_->setText("RAM: " + value);
        else if (key == "CPU_TEMP") cpuTempLabel_->setText("CPU Temp: " + value);
        else if (key == "GPU") gpuLabel_->setText("GPU: " + value);
        else if (key == "BATTERY") batteryLabel_->setText("Battery: " + value);
        else if (key == "LOAD") loadLabel_->setText("Load: " + value);
        else if (key == "UPTIME") uptimeLabel_->setText("Uptime: " + value);
    }
}

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
    return path.isEmpty() ? "arch-manager" : path;
}
