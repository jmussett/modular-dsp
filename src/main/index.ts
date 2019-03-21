import { app, BrowserWindow } from 'electron'
import * as path from 'path'
import { format as formatUrl } from 'url'

let mainWindow: Electron.BrowserWindow | null

const isDevelopment = process.env.NODE_ENV !== 'production'

function initialize () {
    function createWindow() {
        mainWindow = new BrowserWindow({
            width: 800,
            height: 600,
            webPreferences: {
                nodeIntegration: true
            }
        })

        if (isDevelopment) {
            mainWindow.loadURL(`http://localhost:${process.env.ELECTRON_WEBPACK_WDS_PORT}`)
        } else {
            mainWindow.loadURL(formatUrl({
                pathname: path.join(__dirname, 'index.html'),
                protocol: 'file',
                slashes: true
            }))
        }

        mainWindow.webContents.openDevTools()

        mainWindow.on('close', app.quit)
        mainWindow.on('closed', () => {
            mainWindow = null
        })
    }

    app.on('ready', createWindow)

    app.on('window-all-closed', () => {
        if (process.platform !== 'darwin') {
            app.quit()
        }
    })

    app.on('activate', () => {
        if (mainWindow === null) {
            createWindow()
        }
    })

    console.log(`Electron Version ${app.getVersion()}`)
}

initialize()
