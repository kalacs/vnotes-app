import App from './App.svelte';
import { appWindow } from '@tauri-apps/api/window'

document.body.addEventListener('click', () => {
// emit an event that are only visible to the current window
appWindow.emit('event-name', { message: 'Tauri is awesome!' })
})

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;