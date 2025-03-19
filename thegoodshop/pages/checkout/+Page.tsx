import { useCart } from '../../src/contexts/CartContext';
import { PayPalScriptProvider, PayPalButtons } from '@paypal/react-paypal-js';
import type { CreateOrderActions, OnApproveActions } from '@paypal/paypal-js';

export default function CheckoutPage() {
  const { items } = useCart();

  // Compute total amount as a string with two decimals.
  const totalAmount = items
    .reduce((sum, item) => sum + item.product.price * item.quantity, 0)
    .toFixed(2);

  // Use environment variable for PayPal client ID if desired.
  const initialOptions = {
    clientId: import.meta.env.VITE_PAYPAL_CLIENT_ID,
    currency: "USD",
    intent: "capture",
  };

  return (
    <div className="p-8 flex flex-col items-center">
      {/* Order Summary Card */}
      <div className="w-full max-w-4xl">
        <div className="bg-white shadow rounded-lg p-6">
          <h2 className="text-2xl font-semibold mb-4">Order Summary</h2>
          <div className="space-y-4">
            {items.map(item => (
              <div
                key={item.product.id}
                className="flex items-center space-x-4 border p-4 rounded-md"
              >
                {/* Placeholder for Product Image */}
                <div className="w-16 h-16 bg-gray-300 flex items-center justify-center rounded">
                  <span className="text-xs text-gray-600">Image</span>
                </div>
                {/* Product details */}
                <div className="flex-1">
                  <h3 className="text-lg font-medium">{item.product.name}</h3>
                  <p className="text-sm text-gray-500">Quantity: {item.quantity}</p>
                </div>
                {/* Price for this item */}
                <div className="text-xl font-bold">
                  ${(item.product.price * item.quantity).toFixed(2)}
                </div>
              </div>
            ))}
          </div>
          <div className="mt-6 text-right">
            <p className="text-2xl font-bold">Total: ${totalAmount}</p>
          </div>
        </div>
      </div>

      {/* PayPal Payment Section */}
      <div className="w-full max-w-md mt-8">
        <PayPalScriptProvider options={initialOptions}>
          <PayPalButtons
            style={{ layout: "vertical" }}
            createOrder={(_data: unknown, actions: CreateOrderActions) => {
              return actions.order.create({
                intent: "CAPTURE",
                purchase_units: [{
                  amount: {
                    currency_code: "USD",
                    value: totalAmount,
                  },
                  payee: {
                    email_address: "ep@memorici.de"
                  }
                }]
              });
            }}
            onApprove={(_data: unknown, actions: OnApproveActions) => {
              return actions.order!.capture().then((details: any) => {
                console.log("Transaction completed by " + details.payer.name.given_name);
                // Optionally clear the cart or redirect to a confirmation page.
              });
            }}
            onError={(err: Record<string, unknown>) => {
              console.error("PayPal Checkout onError", err);
            }}
          />
        </PayPalScriptProvider>
      </div>
    </div>
  );
}
