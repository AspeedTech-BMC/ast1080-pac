#[doc = "Register `OTP_REG164` reader"]
pub type R = crate::R<OtpReg164Spec>;
#[doc = "Register `OTP_REG164` writer"]
pub type W = crate::W<OtpReg164Spec>;
#[doc = "Field `REGREGIONCALIPTRA0STARTOFFSET` reader - REG_REGION_CALIPTRA0_START_OFFSET"]
pub type Regregioncaliptra0startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA0STARTOFFSET` writer - REG_REGION_CALIPTRA0_START_OFFSET"]
pub type Regregioncaliptra0startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONCALIPTRA0SIZE` reader - REG_REGION_CALIPTRA0_SIZE"]
pub type Regregioncaliptra0sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA0SIZE` writer - REG_REGION_CALIPTRA0_SIZE"]
pub type Regregioncaliptra0sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA0_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra0startoffset(&self) -> Regregioncaliptra0startoffsetR {
        Regregioncaliptra0startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA0_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra0size(&self) -> Regregioncaliptra0sizeR {
        Regregioncaliptra0sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA0_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra0startoffset(
        &mut self,
    ) -> Regregioncaliptra0startoffsetW<OtpReg164Spec> {
        Regregioncaliptra0startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA0_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra0size(&mut self) -> Regregioncaliptra0sizeW<OtpReg164Spec> {
        Regregioncaliptra0sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg164Spec;
impl crate::RegisterSpec for OtpReg164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg164::R`](R) reader structure"]
impl crate::Readable for OtpReg164Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg164::W`](W) writer structure"]
impl crate::Writable for OtpReg164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG164 to value 0x0380_0000"]
impl crate::Resettable for OtpReg164Spec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
