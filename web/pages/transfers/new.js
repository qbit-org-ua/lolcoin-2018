import Link from 'next/link'
import Router from 'next/router'

import { LOLCOINS_API_SERVER_URL } from '../../components/constants'

import Layout from '../../components/Layout'
import Loading from '../../components/Loading'


class TransferForm extends React.Component {
  state = { amount: 3, accessToken: '', isPasswordVisible: false }

  render() {
    return (
      <form onSubmit={this.transfer}>
        <label className="d-block">
          <span>{`Получатель:`} </span>
          <b>{this.props.recipient.full_name}</b>
        </label>
        <label className="d-block">
          <span>{`Количество отправляемых ЛОЛкоинов:`} </span>
          <input type="text" value={this.state.amount} onChange={this.handleAmountChange} />
        </label>
	<div>
          <label>
            <span>{`Ваш секретный ключ:`} </span>
            <input type={this.state.isPasswordVisible ? "text" : "password"} value={this.state.accessToken} onChange={this.handleAccessTokenChange} />
          </label>
	  <label className="pl-2">
	    <input type="checkbox" value={this.state.isPasswordVisible} onChange={this.togglePasswordVisibility} />
	    <span> {`Показать пароль`}</span>
	   </label>
	</div>
        <button className="btn btn-primary">Отправить</button>
        <Link href={{ pathname: '/' }}><button className="btn btn-link" type="reset">Вернуться на главную</button></Link>
      </form>
    )
  }

  handleAmountChange = (event) => {
    let amount = parseInt(event.target.value)
    if (isNaN(amount)) {
      amount = ''
    }
    this.setState({ amount })
  }

  handleAccessTokenChange = (event) => {
    this.setState({ accessToken: event.target.value })
  }

  togglePasswordVisibility = () => {
    this.setState((state) => ({ isPasswordVisible: !state.isPasswordVisible }))
  }

  transfer = (event) => {
    event.preventDefault()
    fetch(
      `${LOLCOINS_API_SERVER_URL}/transfers/`,
      {
        headers: {
          'Authorization': encodeURI(this.state.accessToken),
          'Content-Type': 'application/json',
        },
        method: 'POST',
        body: JSON.stringify({
          to: this.props.recipient.user_id,
          amount: this.state.amount,
        })
      }
    )
      .then((response) => response.json())
      .then((user) => {
        Router.push({ pathname: '/' })
      })
  }
}

export default class extends React.Component {
  state = { user: null, accessToken: '' }

  static getInitialProps({ query: { user_id } }) {
    return { userId: user_id }
  }

  componentDidMount() {
    this.loadUserInfo()
  }

  render() {
    return (
      <Layout>
        <h1 className="pt-5 pb-5">Отправка ЛОЛкоинов</h1>
        {this.state.user === null ? <Loading /> : <TransferForm recipient={this.state.user} />}
      </Layout>
    )
  }

  loadUserInfo = () => {
    fetch(`${LOLCOINS_API_SERVER_URL}/user?user_id=${this.props.userId}`)
      .then((response) => response.json())
      .then((user) => {
        console.log(user);
        this.setState({ user })
      })
  }
}
