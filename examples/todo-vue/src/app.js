import Vue from "vue";
import App from "./App.vue";
import { init } from "./rpc";

const vm = new Vue({
	el: "#app",
	data: () => ({
		tasks: [],
	}),
	render: function (h) {
		return h(App, { attrs: { tasks: this.tasks } });
	},
});

window.onload = () => {
	init();
};

function fromRust(tasks) {
	vm.tasks = tasks;
}

export { fromRust };
