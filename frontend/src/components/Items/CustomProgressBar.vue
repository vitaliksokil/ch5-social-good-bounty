<template>
  <div class="custom-progress-wrap">
    <span class="caption p-2" style="background-color:rgba(0, 0, 0, 0.5); border-radius: 3px">Tickets bought: {{currentValue}}/{{maxValue}}</span>
    <div class="custom-progress-inner">
      <div class="custom-progress bg-danger" :style="{'width': width + '%'}"></div>
    </div>
  </div>
</template>

<script>
export default {
  name: "CustomProgressBar",
  props:['currentValue','maxValue'],
  computed:{
    width(){
      return (this.currentValue / this.maxValue) * 100
    }
  }
}
</script>

<style scoped>
.custom-progress-wrap {
  position: relative;
  bottom: 0;
  width: 100%; }
.custom-progress-wrap .custom-progress-inner {
  height: 7px;
  width: 100%;
  background: #ccc; }
.custom-progress-wrap .custom-progress-inner .custom-progress {
  background: #00a651;
  height: 7px;
  border-top-right-radius: 4px;
  border-bottom-right-radius: 4px; }
.custom-progress-wrap .caption {
  position: absolute;
  bottom: 10px;
  right: 10px;
  color: #fff;
  font-size: 12px; }
</style>