import App from './App.svelte';

const app = new App({
	target: document.querySelector('section'),
	props: {
		name: 'world'
	}
});

export default app;