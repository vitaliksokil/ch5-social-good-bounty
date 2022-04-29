const filters = {
    formatTicketPrice(price) {
        return price / (10 ** 24)
    }
}
export default filters;
