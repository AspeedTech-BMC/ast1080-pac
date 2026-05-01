#[doc = "Register `OTP_REG15C` reader"]
pub type R = crate::R<OtpReg15cSpec>;
#[doc = "Register `OTP_REG15C` writer"]
pub type W = crate::W<OtpReg15cSpec>;
#[doc = "Field `REGREGIONUSR3STARTOFFSET` reader - REG_REGION_USR3_START_OFFSET"]
pub type Regregionusr3startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR3STARTOFFSET` writer - REG_REGION_USR3_START_OFFSET"]
pub type Regregionusr3startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONUSR3SIZE` reader - REG_REGION_USR3_SIZE"]
pub type Regregionusr3sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONUSR3SIZE` writer - REG_REGION_USR3_SIZE"]
pub type Regregionusr3sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_USR3_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr3startoffset(&self) -> Regregionusr3startoffsetR {
        Regregionusr3startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR3_SIZE"]
    #[inline(always)]
    pub fn regregionusr3size(&self) -> Regregionusr3sizeR {
        Regregionusr3sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_USR3_START_OFFSET"]
    #[inline(always)]
    pub fn regregionusr3startoffset(&mut self) -> Regregionusr3startoffsetW<OtpReg15cSpec> {
        Regregionusr3startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_USR3_SIZE"]
    #[inline(always)]
    pub fn regregionusr3size(&mut self) -> Regregionusr3sizeW<OtpReg15cSpec> {
        Regregionusr3sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_USR\\_3\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg15cSpec;
impl crate::RegisterSpec for OtpReg15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg15c::R`](R) reader structure"]
impl crate::Readable for OtpReg15cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg15c::W`](W) writer structure"]
impl crate::Writable for OtpReg15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG15C to value 0x0600_0000"]
impl crate::Resettable for OtpReg15cSpec {
    const RESET_VALUE: u32 = 0x0600_0000;
}
