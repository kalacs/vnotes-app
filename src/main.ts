import App from './App.svelte';
import { invoke } from '@tauri-apps/api/tauri'

document.body.addEventListener('click', () => {
	invoke('open_window');
})

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;