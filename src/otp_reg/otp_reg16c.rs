#[doc = "Register `OTP_REG16C` reader"]
pub type R = crate::R<OtpReg16cSpec>;
#[doc = "Register `OTP_REG16C` writer"]
pub type W = crate::W<OtpReg16cSpec>;
#[doc = "Field `REGREGIONCALIPTRA1STARTOFFSET` reader - REG_REGION_CALIPTRA1_START_OFFSET"]
pub type Regregioncaliptra1startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA1STARTOFFSET` writer - REG_REGION_CALIPTRA1_START_OFFSET"]
pub type Regregioncaliptra1startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONCALIPTRA1SIZE` reader - REG_REGION_CALIPTRA1_SIZE"]
pub type Regregioncaliptra1sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONCALIPTRA1SIZE` writer - REG_REGION_CALIPTRA1_SIZE"]
pub type Regregioncaliptra1sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA1_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra1startoffset(&self) -> Regregioncaliptra1startoffsetR {
        Regregioncaliptra1startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA1_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra1size(&self) -> Regregioncaliptra1sizeR {
        Regregioncaliptra1sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_CALIPTRA1_START_OFFSET"]
    #[inline(always)]
    pub fn regregioncaliptra1startoffset(
        &mut self,
    ) -> Regregioncaliptra1startoffsetW<OtpReg16cSpec> {
        Regregioncaliptra1startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_CALIPTRA1_SIZE"]
    #[inline(always)]
    pub fn regregioncaliptra1size(&mut self) -> Regregioncaliptra1sizeW<OtpReg16cSpec> {
        Regregioncaliptra1sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_1\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg16cSpec;
impl crate::RegisterSpec for OtpReg16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg16c::R`](R) reader structure"]
impl crate::Readable for OtpReg16cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg16c::W`](W) writer structure"]
impl crate::Writable for OtpReg16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG16C to value 0x0380_0000"]
impl crate::Resettable for OtpReg16cSpec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
