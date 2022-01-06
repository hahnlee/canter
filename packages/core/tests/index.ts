import { launch } from '../src/launcher'

const browser = launch()
const page = browser.pages()[0]
page.goto('https://google.com')
