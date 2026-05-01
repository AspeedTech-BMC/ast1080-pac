#[doc = "Register `I3CPHYCTRLREG018` reader"]
pub type R = crate::R<I3cphyctrlreg018Spec>;
#[doc = "Register `I3CPHYCTRLREG018` writer"]
pub type W = crate::W<I3cphyctrlreg018Spec>;
#[doc = "Field `REGI2CODFMPSTOPHCNT` reader - REG_I2C_OD_FMP_STOP_HCNT"]
pub type Regi2codfmpstophcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPSTOPHCNT` writer - REG_I2C_OD_FMP_STOP_HCNT"]
pub type Regi2codfmpstophcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMPSTARTHCNT` reader - REG_I2C_OD_FMP_START_HCNT"]
pub type Regi2codfmpstarthcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPSTARTHCNT` writer - REG_I2C_OD_FMP_START_HCNT"]
pub type Regi2codfmpstarthcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_STOP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpstophcnt(&self) -> Regi2codfmpstophcntR {
        Regi2codfmpstophcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_START_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpstarthcnt(&self) -> Regi2codfmpstarthcntR {
        Regi2codfmpstarthcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_STOP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpstophcnt(&mut self) -> Regi2codfmpstophcntW<I3cphyctrlreg018Spec> {
        Regi2codfmpstophcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_START_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpstarthcnt(&mut self) -> Regi2codfmpstarthcntW<I3cphyctrlreg018Spec> {
        Regi2codfmpstarthcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FMP\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg018Spec;
impl crate::RegisterSpec for I3cphyctrlreg018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg018::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg018Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg018::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG018 to value 0x0033_0063"]
impl crate::Resettable for I3cphyctrlreg018Spec {
    const RESET_VALUE: u32 = 0x0033_0063;
}
