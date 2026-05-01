#[doc = "Register `OTP_REG174` reader"]
pub type R = crate::R<OtpReg174Spec>;
#[doc = "Register `OTP_REG174` writer"]
pub type W = crate::W<OtpReg174Spec>;
#[doc = "Field `REGREGIONCALIPTRA2STARTOFFSET` reader - REG_REGION_CALIPTRA2_START_OFFSET"]
pub type Regregioncaliptra2startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA2STARTOFFSET` writer - REG_REGION_CALIPTRA2_START_OFFSET"]
pub type Regregioncaliptra2startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONCALIPTRA2SIZE` reader - REG_REGION_CALIPTRA2_SIZE"]
pub type Regregioncaliptra2sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA2SIZE` writer - REG_REGION_CALIPTRA2_SIZE"]
pub type Regregioncaliptra2sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA2_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra2startoffset(&self) -> Regregioncaliptra2startoffsetR {
        Regregioncaliptra2startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA2_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra2size(&self) -> Regregioncaliptra2sizeR {
        Regregioncaliptra2sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA2_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra2startoffset(
        &mut self,
    ) -> Regregioncaliptra2startoffsetW<OtpReg174Spec> {
        Regregioncaliptra2startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA2_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra2size(&mut self) -> Regregioncaliptra2sizeW<OtpReg174Spec> {
        Regregioncaliptra2sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_2\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg174Spec;
impl crate::RegisterSpec for OtpReg174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg174::R`](R) reader structure"]
impl crate::Readable for OtpReg174Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg174::W`](W) writer structure"]
impl crate::Writable for OtpReg174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG174 to value 0x0380_0000"]
impl crate::Resettable for OtpReg174Spec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
