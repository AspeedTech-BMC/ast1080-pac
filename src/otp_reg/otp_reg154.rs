#[doc = "Register `OTP_REG154` reader"]
pub type R = crate::R<OtpReg154Spec>;
#[doc = "Register `OTP_REG154` writer"]
pub type W = crate::W<OtpReg154Spec>;
#[doc = "Field `REGREGIONUSR2STARTOFFSET` reader - REG_REGION_USR2_START_OFFSET"]
pub type Regregionusr2startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR2STARTOFFSET` writer - REG_REGION_USR2_START_OFFSET"]
pub type Regregionusr2startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONUSR2SIZE` reader - REG_REGION_USR2_SIZE"]
pub type Regregionusr2sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR2SIZE` writer - REG_REGION_USR2_SIZE"]
pub type Regregionusr2sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_USR2_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr2startoffset(&self) -> Regregionusr2startoffsetR {
        Regregionusr2startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR2_SIZE"]
    #[inline(always)]
    pub fn regregionusr2size(&self) -> Regregionusr2sizeR {
        Regregionusr2sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_USR2_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr2startoffset(&mut self) -> Regregionusr2startoffsetW<OtpReg154Spec> {
        Regregionusr2startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR2_SIZE"]
    #[inline(always)]
    pub fn regregionusr2size(&mut self) -> Regregionusr2sizeW<OtpReg154Spec> {
        Regregionusr2sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_2\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg154Spec;
impl crate::RegisterSpec for OtpReg154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg154::R`](R) reader structure"]
impl crate::Readable for OtpReg154Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg154::W`](W) writer structure"]
impl crate::Writable for OtpReg154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG154 to value 0x0600_0000"]
impl crate::Resettable for OtpReg154Spec {
    const RESET_VALUE: u32 = 0x0600_0000;
}
