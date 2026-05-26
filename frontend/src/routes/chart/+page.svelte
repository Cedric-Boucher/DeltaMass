<script lang="ts">
	import type { Mass } from '$lib/types';
	import { selectedUnit, UNIT_LABELS, convertFromKg } from '$lib/stores/units';
	import * as echarts from 'echarts';

	let { data }: { data: { masses: Mass[] } } = $props();

	let isDark = $state(false);
	$effect(() => {
		if (typeof document !== 'undefined') {
			isDark = document.documentElement.classList.contains('dark');
			const observer = new MutationObserver(() => {
				isDark = document.documentElement.classList.contains('dark');
			});
			observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
			return () => observer.disconnect();
		}
	});

	let chartData = $derived(
		[...data.masses]
			.sort(
				(a, b) =>
					new Date(a.measurement_timestamp).getTime() - new Date(b.measurement_timestamp).getTime()
			)
			.map((item) => [item.measurement_timestamp, convertFromKg(item.mass_kg, $selectedUnit)])
	);

	let chartConfig = $derived({
		data: chartData,
		unit: $selectedUnit,
		isDark: isDark
	});

	function echartsAction(node: HTMLElement, config: typeof chartConfig) {
		const chart = echarts.init(node);

		function updateChart(currentConfig: typeof chartConfig) {
			const { data: currentData, unit, isDark } = currentConfig;

			// Theme-dependent design palettes matching your tailwind configuration
			const textColor = isDark ? '#9CA3AF' : '#374151'; // gray-400 vs gray-700
			const splitLineColor = isDark ? '#374151' : '#E5E7EB'; // gray-700 vs gray-200
			const tooltipBg = isDark ? '#1F2937' : '#FFFFFF'; // gray-800 vs white
			const tooltipBorder = isDark ? '#4B5563' : '#E5E7EB'; // gray-600 vs gray-200

			const option: echarts.EChartsOption = {
				backgroundColor: 'transparent',
				title: {
					text: `Weight History (${UNIT_LABELS[unit]})`,
					left: 'center',
					textStyle: { color: textColor }
				},
				tooltip: {
					trigger: 'axis',
					backgroundColor: tooltipBg,
					borderColor: tooltipBorder,
					textStyle: { color: textColor },
					// eslint-disable-next-line @typescript-eslint/no-explicit-any
					formatter: function (params: any) {
						const date = new Date(params[0].value[0]);
						const value = params[0].value[1];

						const formattedValue = value.toLocaleString(undefined, {
							minimumFractionDigits: 1,
							maximumFractionDigits: 2
						});

						return `
                            <div class="font-sans p-1">
                                <b style="color: ${isDark ? '#9CA3AF' : '#6B7280'}">
                                    ${date.toLocaleDateString()} ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                </b><br/>
                                <span style="color: #2563EB; font-weight: bold;">●</span> 
                                <span style="font-weight: bold; color: ${isDark ? '#F3F4F6' : '#111827'}">
                                    ${formattedValue} ${UNIT_LABELS[unit]}
                                </span>
                            </div>
                        `;
					}
				},
				grid: {
					left: '4%',
					right: '4%',
					bottom: '15%',
					containLabel: true
				},
				xAxis: {
					type: 'time',
					name: 'Date',
					boundaryGap: [0, 0],
					axisLabel: { color: textColor },
					splitLine: { show: true, lineStyle: { color: splitLineColor, type: 'dashed' } }
				},
				yAxis: {
					type: 'value',
					name: UNIT_LABELS[unit],
					nameTextStyle: { color: textColor },
					scale: true,
					axisLabel: { color: textColor },
					splitLine: { show: true, lineStyle: { color: splitLineColor } }
				},
				dataZoom: [
					{ type: 'inside', start: 0, end: 100 },
					{
						type: 'slider',
						start: 0,
						end: 100,
						textStyle: { color: textColor },
						borderColor: splitLineColor
					}
				],
				series: [
					{
						name: 'Weight',
						type: 'line',
						data: currentData,

						smooth: false,
						lineStyle: {
							width: 0,
							color: '#2563EB'
						},
						symbol: 'circle',
						symbolSize: 8,
						showSymbol: true,
						itemStyle: { color: '#2563EB' }
					}
				]
			};
			chart.setOption(option, true);
		}

		updateChart(config);

		const resizeObserver = new ResizeObserver(() => {
			chart.resize();
		});
		resizeObserver.observe(node);

		return {
			update(nextConfig: typeof chartConfig) {
				updateChart(nextConfig);
			},
			destroy() {
				resizeObserver.disconnect();
				chart.dispose();
			}
		};
	}
</script>

<div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
	<div>
		<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Mass Chart</h1>
		<p class="text-sm text-gray-500 dark:text-gray-400">
			Scroll/pinch to zoom, drag to pan across dates.
		</p>
	</div>
</div>

{#if chartData.length > 0}
	<div
		class="bg-white dark:bg-gray-800 p-4 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm w-full"
	>
		<div use:echartsAction={chartConfig} class="w-full h-[500px]"></div>
	</div>
{:else}
	<div
		class="bg-gray-50 dark:bg-gray-900 rounded-xl p-8 text-center border-2 border-dashed border-gray-200 dark:border-gray-700 w-full"
	>
		<p class="text-gray-500 dark:text-gray-400 italic">
			Add weight readings to see historical trends.
		</p>
	</div>
{/if}
