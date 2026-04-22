#[doc = "Register `SPIPF004` reader"]
pub type R = crate::R<Spipf004Spec>;
#[doc = "Register `SPIPF004` writer"]
pub type W = crate::W<Spipf004Spec>;
#[doc = "Field `INTStatusOfCmdBlock` reader - Interrupt Status of Command Block"]
pub type IntstatusOfCmdBlockR = crate::BitReader;
#[doc = "Field `INTStatusOfCmdBlock` writer - Interrupt Status of Command Block"]
pub type IntstatusOfCmdBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTStatusOfWrBlock` reader - Interrupt Status of Write Block"]
pub type IntstatusOfWrBlockR = crate::BitReader;
#[doc = "Field `INTStatusOfWrBlock` writer - Interrupt Status of Write Block"]
pub type IntstatusOfWrBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTStatusOfReadBlock` reader - Interrupt Status of Read Block"]
pub type IntstatusOfReadBlockR = crate::BitReader;
#[doc = "Field `INTStatusOfReadBlock` writer - Interrupt Status of Read Block"]
pub type IntstatusOfReadBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `EnblINTOfCmdBlock` reader - Enable Interrupt of Command Block"]
pub type EnblIntofCmdBlockR = crate::BitReader;
#[doc = "Field `EnblINTOfCmdBlock` writer - Enable Interrupt of Command Block"]
pub type EnblIntofCmdBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblINTOfWrBlock` reader - Enable Interrupt of Write Block"]
pub type EnblIntofWrBlockR = crate::BitReader;
#[doc = "Field `EnblINTOfWrBlock` writer - Enable Interrupt of Write Block"]
pub type EnblIntofWrBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblINTOfReadBlock` reader - Enable Interrupt of Read Block"]
pub type EnblIntofReadBlockR = crate::BitReader;
#[doc = "Field `EnblINTOfReadBlock` writer - Enable Interrupt of Read Block"]
pub type EnblIntofReadBlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Status of Command Block"]
    #[inline(always)]
    pub fn intstatus_of_cmd_block(&self) -> IntstatusOfCmdBlockR {
        IntstatusOfCmdBlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of Write Block"]
    #[inline(always)]
    pub fn intstatus_of_wr_block(&self) -> IntstatusOfWrBlockR {
        IntstatusOfWrBlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of Read Block"]
    #[inline(always)]
    pub fn intstatus_of_read_block(&self) -> IntstatusOfReadBlockR {
        IntstatusOfReadBlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - Enable Interrupt of Command Block"]
    #[inline(always)]
    pub fn enbl_intof_cmd_block(&self) -> EnblIntofCmdBlockR {
        EnblIntofCmdBlockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Interrupt of Write Block"]
    #[inline(always)]
    pub fn enbl_intof_wr_block(&self) -> EnblIntofWrBlockR {
        EnblIntofWrBlockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Interrupt of Read Block"]
    #[inline(always)]
    pub fn enbl_intof_read_block(&self) -> EnblIntofReadBlockR {
        EnblIntofReadBlockR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Status of Command Block"]
    #[inline(always)]
    pub fn intstatus_of_cmd_block(&mut self) -> IntstatusOfCmdBlockW<Spipf004Spec> {
        IntstatusOfCmdBlockW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Status of Write Block"]
    #[inline(always)]
    pub fn intstatus_of_wr_block(&mut self) -> IntstatusOfWrBlockW<Spipf004Spec> {
        IntstatusOfWrBlockW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Status of Read Block"]
    #[inline(always)]
    pub fn intstatus_of_read_block(&mut self) -> IntstatusOfReadBlockW<Spipf004Spec> {
        IntstatusOfReadBlockW::new(self, 2)
    }
    #[doc = "Bit 16 - Enable Interrupt of Command Block"]
    #[inline(always)]
    pub fn enbl_intof_cmd_block(&mut self) -> EnblIntofCmdBlockW<Spipf004Spec> {
        EnblIntofCmdBlockW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Interrupt of Write Block"]
    #[inline(always)]
    pub fn enbl_intof_wr_block(&mut self) -> EnblIntofWrBlockW<Spipf004Spec> {
        EnblIntofWrBlockW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Interrupt of Read Block"]
    #[inline(always)]
    pub fn enbl_intof_read_block(&mut self) -> EnblIntofReadBlockW<Spipf004Spec> {
        EnblIntofReadBlockW::new(self, 18)
    }
}
#[doc = "Interrupt Enable and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf004Spec;
impl crate::RegisterSpec for Spipf004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf004::R`](R) reader structure"]
impl crate::Readable for Spipf004Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf004::W`](W) writer structure"]
impl crate::Writable for Spipf004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF004 to value 0"]
impl crate::Resettable for Spipf004Spec {}
