#[doc = "Register `OTP_REG134` reader"]
pub type R = crate::R<OtpReg134Spec>;
#[doc = "Register `OTP_REG134` writer"]
pub type W = crate::W<OtpReg134Spec>;
#[doc = "Field `REGREGIONSECURE2STARTOFFSET` reader - REG_REGION_SECURE2_START_OFFSET"]
pub type Regregionsecure2startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE2STARTOFFSET` writer - REG_REGION_SECURE2_START_OFFSET"]
pub type Regregionsecure2startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONSECURE2SIZE` reader - REG_REGION_SECURE2_SIZE"]
pub type Regregionsecure2sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE2SIZE` writer - REG_REGION_SECURE2_SIZE"]
pub type Regregionsecure2sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_SECURE2_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure2startoffset(&self) -> Regregionsecure2startoffsetR {
        Regregionsecure2startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE2_SIZE"]
    #[inline(always)]
    pub fn regregionsecure2size(&self) -> Regregionsecure2sizeR {
        Regregionsecure2sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_SECURE2_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure2startoffset(&mut self) -> Regregionsecure2startoffsetW<OtpReg134Spec> {
        Regregionsecure2startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE2_SIZE"]
    #[inline(always)]
    pub fn regregionsecure2size(&mut self) -> Regregionsecure2sizeW<OtpReg134Spec> {
        Regregionsecure2sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg134Spec;
impl crate::RegisterSpec for OtpReg134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg134::R`](R) reader structure"]
impl crate::Readable for OtpReg134Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg134::W`](W) writer structure"]
impl crate::Writable for OtpReg134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG134 to value 0x0090_0000"]
impl crate::Resettable for OtpReg134Spec {
    const RESET_VALUE: u32 = 0x0090_0000;
}
