// Colors

export interface RGB {
  r: number;
  g: number;
  b: number;
}

export interface RGBW extends RGB {
  w: number;
}

export interface HSV {
  h: number;
  s: number;
  v: number;
}

export interface HSL {
  h: number;
  s: number;
  l: number;
}

// Hardware

export interface Device {
  hardware: Hardware;
  serialPort: string;
}

export interface Hardware {
  info: Info;
  usb: Usb;
  bootloader: boolean;
  keyboard?: Grid;
  keyboardUnderglow?: Grid;
  rgbwMode: boolean;
  wireless: boolean;
  instructions: Languages;
  virtualInfo?: Virtual;
}

export interface Virtual {
  version: VirtualNode;
  keymapCustom: VirtualNode;
  keymapDefault: VirtualNode;
  keymapOnlyCustom: VirtualNode;
  settingsDefaultLayer: VirtualNode;
  settingsValid: VirtualNode;
  settingsVersion: VirtualNode;
  settingsCrc: VirtualNode;
  eepromContents: VirtualNode;
  eepromFree: VirtualNode;
  ledAt: VirtualNode;
  ledSetAll: VirtualNode;
  ledMode: VirtualNode;
  ledFade?: VirtualNode;
  ledBrightness: VirtualNode;
  ledBrightnessWireless?: VirtualNode;
  ledBrightnessUg: VirtualNode;
  ledBrightnessUgWireless?: VirtualNode;
  ledTheme: VirtualNode;
  palette: VirtualNode;
  colormapMap: VirtualNode;
  idleLedsTimeLimit: VirtualNode;
  idleLedsWireless?: VirtualNode;
  hardwareVersion: VirtualNode;
  hardwareSidePower: VirtualNode;
  hardwareSideVer: VirtualNode;
  hardwareSledVer: VirtualNode;
  hardwareSledCurrent: VirtualNode;
  hardwareLayout: VirtualNode;
  hardwareJoint: VirtualNode;
  hardwareKeyscan: VirtualNode;
  hardwareCrcErrors: VirtualNode;
  hardwareFirmware: VirtualNode;
  hardwareChipId: VirtualNode;
  qukeysHoldTimeout: VirtualNode;
  qukeysOverlapThreshold: VirtualNode;
  superkeysMap: VirtualNode;
  superkeysWaitFor: VirtualNode;
  superkeysTimeout: VirtualNode;
  superkeysRepeat: VirtualNode;
  superkeysHoldStart: VirtualNode;
  superkeysOverlap: VirtualNode;
  macrosMap: VirtualNode;
  macrosTrigger: VirtualNode;
  macrosMemory: VirtualNode;
  help: VirtualNode;
  mouseSpeed: VirtualNode;
  mouseSpeedDelay: VirtualNode;
  mouseAccelSpeed: VirtualNode;
  mouseAccelDelay: VirtualNode;
  mouseWheelSpeed: VirtualNode;
  mouseWheelDelay: VirtualNode;
  mouseSpeedLimit: VirtualNode;
  layerActivate: VirtualNode;
  layerDeactivate: VirtualNode;
  layerIsActive: VirtualNode;
  layerMoveTo: VirtualNode;
  layerState: VirtualNode;
  wirelessBatteryLeftLevel?: VirtualNode;
  wirelessBatteryRightLevel?: VirtualNode;
  wirelessBatteryLeftStatus?: VirtualNode;
  wirelessBatteryRightStatus?: VirtualNode;
  wirelessBatterySavingMode?: VirtualNode;
  wirelessEnergyModes?: VirtualNode;
  wirelessEnergyDisable?: VirtualNode;
  wirelessEnergyCurrentMode?: VirtualNode;
  wirelessBluetoothMacs?: VirtualNode;
  wirelessBluetoothPeerIds?: VirtualNode;
  wirelessBluetoothRemove?: VirtualNode;
  wirelessBluetoothDeviceName?: VirtualNode;
  wirelessBluetoothList?: VirtualNode;
  wirelessRfPower?: VirtualNode;
  wirelessRfStability?: VirtualNode;
  wirelessRfChannelHop?: VirtualNode;
  wirelessRfSyncPairing?: VirtualNode;
}

export interface VirtualNode {
  data: string;
  erasable: boolean;
}

export interface Info {
  vendor: Vendor;
  product: Product;
  keyboardType: DeviceType;
  displayName: string;
  urls: Urls;
}

export interface Urls {
  homepage: Url;
}

export interface Url {
  name: string;
  url: string;
}

export enum Vendor {
  Dygma = "Dygma",
}

export enum Product {
  Defy = "Defy",
  Raise = "Raise",
  Raise2 = "Raise 2",
}

export enum DeviceType {
  Wired = "Wired",
  Wireless = "Wireless",
  ISO = "ISO",
  ANSI = "ANSI",
}

export interface Usb {
  vendorId: number;
  productId: number;
}

export interface Grid {
  rows: number;
  columns: number;
}

export interface Languages {
  en: Dialog;
}

export interface Dialog {
  updateInstructions: string;
}

// Settings

export interface Settings {
  keymapCustom: number[];
  keymapDefault: number[];
  keymapOnlyCustom: boolean;
  settingsDefaultLayer: number;
  superkeysMap: number[];
  superkeysWaitFor: number;
  superkeysTimeout: number;
  superkeysRepeat: number;
  superkeysHoldStart: number;
  superkeysOverlap: number;
  ledMode: LedMode;
  ledBrightnessKeysWired: number;
  ledBrightnessUnderglowWired?: number;
  ledBrightnessKeysWireless?: number;
  ledBrightnessUnderglowWireless?: number;
  ledFade?: number;
  ledTheme: RGB[];
  paletteRgb?: RGB[];
  paletteRgbw?: RGBW[];
  colorMap: number[];
  ledIdleTrueSleep?: boolean;
  ledIdleTrueSleepTime?: number;
  ledIdleTimeLimitWired: number;
  ledIdleTimeLimitWireless?: number;
  qukeysHoldTimeout: number;
  qukeysOverlapThreshold: number;
  macrosMap: number[];
  mouseSpeed: number;
  mouseDelay: number;
  mouseAccelerationSpeed: number;
  mouseAccelerationDelay: number;
  mouseWheelSpeed: number;
  mouseWheelDelay: number;
  mouseSpeedLimit: number;
  wirelessBatterySavingMode?: boolean;
  wirelessRfPowerLevel?: WirelessPowerMode;
  wirelessRfChannelHop?: boolean;
}

export enum LedMode {
  /** The default mode. The LEDs will be set to the color of the layer you are on. */
  Static = 0,
  /** Rainbow effect. */
  Rainbow = 1,
  /** Cycle colors. */
  Cycle = 2,
  /** All LEDs will be off until pressed, they will light up when pressed and cycle colors back to off. */
  Stalker = 3,
  /** All LEDs to red. */
  Red = 4,
  /** All LEDs to green. */
  Green = 5,
  /** All LEDs to blue. */
  Blue = 6,
  /** All LEDs to white. */
  White = 7,
  /** All LEDs to off. */
  Off = 8,
  /** The inner three LEDs on both sides will be green, the rest will be off. */
  Debug = 9,
  /** Emulates the bluetooth connect sequence. */
  Bluetooth = 10,
}

/** The wireless power mode states. */
export enum WirelessPowerMode {
  /** Low power mode. The battery will last longer but the wireless range will be shorter. */
  Low = 0,
  /** Medium power mode. The battery will last a bit less but the wireless range will be longer. */
  Medium = 1,
  /** High power mode. The battery will last the least but the wireless range will be the longest. */
  High = 2,
}

/** The device side. */
export enum Side {
  Right = 0,
  Left = 1,
}
