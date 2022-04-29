<script>
	import { Line } from 'svelte-chartjs'
	import {enUS} from 'date-fns/locale'
	import 'chartjs-adapter-date-fns';
</script>


<div>
	<!--<Line data={{
		datasets: [{
			label: "Altitude",
			data: [
				{
					x: 0,
					y: 0,
				},
				{
					x: 10,
					y: 5,
				},
				{
					x: 50,
					y: 8,
				},
				{
					x: 120,
					y: 9,
				},
				{
					x: 130,
					y: 10,
				},

			],
			backgroundColor: "rgba(255, 99, 132, 0.2)",
			borderColor: "rgba(255, 99, 132, 1)",
			borderWidth: 1
		}]
	}}
	options={{
		responsive: true,
		scales: {
			x: {
				type: 'time',
				time: {
					unit: 'seconds',
					parser: 't.SS'
				},
				ticks: {
					source: 'auto'
				},
			},
			adapters: {
				locale: enUS
			}
		}
	}}/>-->

	

</div>