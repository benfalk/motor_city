# frozen_string_literal: true

module MotorCity
  module FFI
    class ConnectionResult < ::FFI::Struct
      layout :status, :uint32,
             :value,  :pointer
    end
  end
end
