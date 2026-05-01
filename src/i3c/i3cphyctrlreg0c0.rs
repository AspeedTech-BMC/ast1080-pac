#[doc = "Register `I3CPHYCTRLREG0C0` reader"]
pub type R = crate::R<I3cphyctrlreg0c0Spec>;
#[doc = "Register `I3CPHYCTRLREG0C0` writer"]
pub type W = crate::W<I3cphyctrlreg0c0Spec>;
#[doc = "Field `REGI2CSDASTUCKLOWPATSCLCNT` reader - REG_I2C_SDA_STUCK_LOW_PAT_SCL_CNT"]
pub type Regi2csdastucklowpatsclcntR = crate::FieldReader;
#[doc = "Field `REGI2CSDASTUCKLOWPATSCLCNT` writer - REG_I2C_SDA_STUCK_LOW_PAT_SCL_CNT"]
pub type Regi2csdastucklowpatsclcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGI3CSDASTUCKPATSCLCNT` reader - REG_I3C_SDA_STUCK_PAT_SCL_CNT"]
pub type Regi3csdastuckpatsclcntR = crate::FieldReader;
#[doc = "Field `REGI3CSDASTUCKPATSCLCNT` writer - REG_I3C_SDA_STUCK_PAT_SCL_CNT"]
pub type Regi3csdastuckpatsclcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGSDASTUCKHOLDSCLHCNT` reader - REG_SDA_STUCK_HOLD_SCLH_CNT"]
pub type RegsdastuckholdsclhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGSDASTUCKHOLDSCLHCNT` writer - REG_SDA_STUCK_HOLD_SCLH_CNT"]
pub type RegsdastuckholdsclhcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - REG_I2C_SDA_STUCK_LOW_PAT_SCL_CNT"]
    #[inline(always)]
    pub fn regi2csdastucklowpatsclcnt(&self) -> Regi2csdastucklowpatsclcntR {
        Regi2csdastucklowpatsclcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_I3C_SDA_STUCK_PAT_SCL_CNT"]
    #[inline(always)]
    pub fn regi3csdastuckpatsclcnt(&self) -> Regi3csdastuckpatsclcntR {
        Regi3csdastuckpatsclcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - REG_SDA_STUCK_HOLD_SCLH_CNT"]
    #[inline(always)]
    pub fn regsdastuckholdsclhcnt(&self) -> RegsdastuckholdsclhcntR {
        RegsdastuckholdsclhcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_I2C_SDA_STUCK_LOW_PAT_SCL_CNT"]
    #[inline(always)]
    pub fn regi2csdastucklowpatsclcnt(
        &mut self,
    ) -> Regi2csdastucklowpatsclcntW<I3cphyctrlreg0c0Spec> {
        Regi2csdastucklowpatsclcntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_I3C_SDA_STUCK_PAT_SCL_CNT"]
    #[inline(always)]
    pub fn regi3csdastuckpatsclcnt(&mut self) -> Regi3csdastuckpatsclcntW<I3cphyctrlreg0c0Spec> {
        Regi3csdastuckpatsclcntW::new(self, 8)
    }
    #[doc = "Bits 16:31 - REG_SDA_STUCK_HOLD_SCLH_CNT"]
    #[inline(always)]
    pub fn regsdastuckholdsclhcnt(&mut self) -> RegsdastuckholdsclhcntW<I3cphyctrlreg0c0Spec> {
        RegsdastuckholdsclhcntW::new(self, 16)
    }
}
#[doc = "SDA\\_STUCK\\_SET1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0c0Spec;
impl crate::RegisterSpec for I3cphyctrlreg0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0c0::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0c0::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0C0 to value 0x7530_0708"]
impl crate::Resettable for I3cphyctrlreg0c0Spec {
    const RESET_VALUE: u32 = 0x7530_0708;
}
