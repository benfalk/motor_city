# frozen_string_literal: true

RSpec.describe MotorCity::FFI do
  describe "#add" do
    let(:left) { 2 }
    let(:right) { 3 }
    subject { described_class.add(left, right) }
    it { is_expected.to eq(5) }
  end
end
