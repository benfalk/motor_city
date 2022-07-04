# frozen_string_literal: true

RSpec.describe MotorCity::FFI do
  describe "#connection_ok" do
    subject { described_class.connection_ok(MotorCity.connection) }
    it { is_expected.to be(true) }
  end
end
