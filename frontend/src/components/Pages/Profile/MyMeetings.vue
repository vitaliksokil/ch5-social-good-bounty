<template>
<div class="row">
  <meeting-list-item-component v-for="(meeting, key) in myMeetings" :key="key"
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
</template>

<script>
import MeetingListItemComponent from "@/components/Pages/MeetingListItemComponent";
export default {
  name: "MyMeetings",
  components:{MeetingListItemComponent},
  data(){
    return {
      myMeetings : {}
    }
  },
  methods:{
    async getMyMeetings(){
      const argsBase64 = window.btoa(JSON.stringify({owner_account_id:window.nearAccount.accountId}))
      let result = await window.provider.query({
        request_type: "call_function",
        account_id: window.walletSelector.getContractId(),
        method_name: "get_meetings_by_owner",
        args_base64: argsBase64,
        finality: "optimistic",
      })
      this.myMeetings = JSON.parse(Buffer.from(result.result).toString())
    }
  },
  async mounted() {
    let loader = this.$loading.show();
    await this.getMyMeetings()
    loader.hide();
  }
}
</script>

<style scoped>

</style>