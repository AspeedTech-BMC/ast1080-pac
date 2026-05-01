#[doc = "Register `OTP_REG17C` reader"]
pub type R = crate::R<OtpReg17cSpec>;
#[doc = "Register `OTP_REG17C` writer"]
pub type W = crate::W<OtpReg17cSpec>;
#[doc = "Field `REGREGIONCALIPTRA3STARTOFFSET` reader - REG_REGION_CALIPTRA3_START_OFFSET"]
pub type Regregioncaliptra3startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA3STARTOFFSET` writer - REG_REGION_CALIPTRA3_START_OFFSET"]
pub type Regregioncaliptra3startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONCALIPTRA3SIZE` reader - REG_REGION_CALIPTRA3_SIZE"]
pub type Regregioncaliptra3sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA3SIZE` writer - REG_REGION_CALIPTRA3_SIZE"]
pub type Regregioncaliptra3sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA3_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra3startoffset(&self) -> Regregioncaliptra3startoffsetR {
        Regregioncaliptra3startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA3_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra3size(&self) -> Regregioncaliptra3sizeR {
        Regregioncaliptra3sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA3_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra3startoffset(
        &mut self,
    ) -> Regregioncaliptra3startoffsetW<OtpReg17cSpec> {
        Regregioncaliptra3startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA3_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra3size(&mut self) -> Regregioncaliptra3sizeW<OtpReg17cSpec> {
        Regregioncaliptra3sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_3\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg17cSpec;
impl crate::RegisterSpec for OtpReg17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg17c::R`](R) reader structure"]
impl crate::Readable for OtpReg17cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg17c::W`](W) writer structure"]
impl crate::Writable for OtpReg17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG17C to value 0x0380_0000"]
impl crate::Resettable for OtpReg17cSpec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
