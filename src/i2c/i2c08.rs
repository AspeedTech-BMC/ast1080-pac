#[doc = "Register `I2C08` reader"]
pub type R = crate::R<I2c08Spec>;
#[doc = "Register `I2C08` writer"]
pub type W = crate::W<I2c08Spec>;
#[doc = "Field `TXBYTEBUFFER` reader - TX_BYTE_BUFFER"]
pub type TxbytebufferR = crate::FieldReader;
#[doc = "Field `RXBYTEBUFFER` reader - RX_BYTE_BUFFER"]
pub type RxbytebufferR = crate::FieldReader;
#[doc = "Field `BUSSTA` reader - BUS_STA"]
pub type BusstaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - TX_BYTE_BUFFER"]
    #[inline(always)]
    pub fn txbytebuffer(&self) -> TxbytebufferR {
        TxbytebufferR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX_BYTE_BUFFER"]
    #[inline(always)]
    pub fn rxbytebuffer(&self) -> RxbytebufferR {
        RxbytebufferR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - BUS_STA"]
    #[inline(always)]
    pub fn bussta(&self) -> BusstaR {
        BusstaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c08Spec;
impl crate::RegisterSpec for I2c08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c08::R`](R) reader structure"]
impl crate::Readable for I2c08Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c08::W`](W) writer structure"]
impl crate::Writable for I2c08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C08 to value 0"]
impl crate::Resettable for I2c08Spec {}
