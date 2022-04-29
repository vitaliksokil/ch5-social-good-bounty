<template>
  <section class="site-section">
    <div class="container">

      <div class="row">
        <meeting-list-item-component v-for="(meeting, key) in meetings" :key="key"
                                     :currentValue="meeting.bought_tickets"
                                     :maxValue="meeting.total_tickets"
                                     :id="key"
                                     :title="meeting.title"
                                     :ticketPrice="meeting.ticket_price"
                                     :author="meeting.owner_name"
                                     :image="meeting.nft_media"
                                     :donation-type="meeting.donation_type"
        ></meeting-list-item-component>
      </div>
    </div>
  </section>
</template>

<script>
import MeetingListItemComponent from "@/components/Pages/MeetingListItemComponent";
export default {
  name: "MeetingsComponent",
  components:{
    MeetingListItemComponent
  },
  data(){
    return {
      meetings : {}
    }
  },
  methods:{
    async getMyMeetings(){
      let result = await window.provider.query({
        request_type: "call_function",
        account_id: window.walletSelector.getContractId(),
        method_name: "get_all_meetings",
        args_base64: '',
        finality: "optimistic",
      })
      this.meetings = JSON.parse(Buffer.from(result.result).toString())
    }
  },
  async mounted() {
    let loader = this.$loading.show();
    await this.getMyMeetings()
    loader.hide()
  },
}
</script>

<style scoped>


</style>