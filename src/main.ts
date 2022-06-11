import FrontController from './FrontController.svelte';

const app = new FrontController({
	target: document.querySelector('body'),
});

export default app;