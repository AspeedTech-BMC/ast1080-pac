#[doc = "Register `UARTIER` reader"]
pub type R = crate::R<UartierSpec>;
#[doc = "Register `UARTIER` writer"]
pub type W = crate::W<UartierSpec>;
#[doc = "Field `ERBFI` reader - Enable Received Data Available Interrupt"]
pub type ErbfiR = crate::BitReader;
#[doc = "Field `ERBFI` writer - Enable Received Data Available Interrupt"]
pub type ErbfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETBEI` reader - Enable Transmitter Holding Register Empty Interrupt"]
pub type EtbeiR = crate::BitReader;
#[doc = "Field `ETBEI` writer - Enable Transmitter Holding Register Empty Interrupt"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSI` reader - Enable Receiver Line Status Interrupt"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `ELSI` writer - Enable Receiver Line Status Interrupt"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDSSI` reader - Enable Modem Status Interrupt"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `EDSSI` writer - Enable Modem Status Interrupt"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&self) -> ErbfiR {
        ErbfiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&mut self) -> ErbfiW<UartierSpec> {
        ErbfiW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&mut self) -> EtbeiW<UartierSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<UartierSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&mut self) -> EdssiW<UartierSpec> {
        EdssiW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartierSpec;
impl crate::RegisterSpec for UartierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartier::R`](R) reader structure"]
impl crate::Readable for UartierSpec {}
#[doc = "`write(|w| ..)` method takes [`uartier::W`](W) writer structure"]
impl crate::Writable for UartierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIER to value 0"]
impl crate::Resettable for UartierSpec {}
