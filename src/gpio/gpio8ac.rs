#[doc = "Register `GPIO8AC` reader"]
pub type R = crate::R<Gpio8acSpec>;
#[doc = "Register `GPIO8AC` writer"]
pub type W = crate::W<Gpio8acSpec>;
#[doc = "Field `GPIO156WrPrivilegeOfMaster` reader - GPIO156 Write Privilege of Master"]
pub type Gpio156wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO156WrPrivilegeOfMaster` writer - GPIO156 Write Privilege of Master"]
pub type Gpio156wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO157WrPrivilegeOfMaster` reader - GPIO157 Write Privilege of Master"]
pub type Gpio157wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO157WrPrivilegeOfMaster` writer - GPIO157 Write Privilege of Master"]
pub type Gpio157wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO158WrPrivilegeOfMaster` reader - GPIO158 Write Privilege of Master"]
pub type Gpio158wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO158WrPrivilegeOfMaster` writer - GPIO158 Write Privilege of Master"]
pub type Gpio158wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO159WrPrivilegeOfMaster` reader - GPIO159 Write Privilege of Master"]
pub type Gpio159wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO159WrPrivilegeOfMaster` writer - GPIO159 Write Privilege of Master"]
pub type Gpio159wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO156 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio156wr_privilege_of_master(&self) -> Gpio156wrPrivilegeOfMasterR {
        Gpio156wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO157 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio157wr_privilege_of_master(&self) -> Gpio157wrPrivilegeOfMasterR {
        Gpio157wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO158 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio158wr_privilege_of_master(&self) -> Gpio158wrPrivilegeOfMasterR {
        Gpio158wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO159 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio159wr_privilege_of_master(&self) -> Gpio159wrPrivilegeOfMasterR {
        Gpio159wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO156 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio156wr_privilege_of_master(&mut self) -> Gpio156wrPrivilegeOfMasterW<Gpio8acSpec> {
        Gpio156wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO157 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio157wr_privilege_of_master(&mut self) -> Gpio157wrPrivilegeOfMasterW<Gpio8acSpec> {
        Gpio157wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO158 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio158wr_privilege_of_master(&mut self) -> Gpio158wrPrivilegeOfMasterW<Gpio8acSpec> {
        Gpio158wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO159 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio159wr_privilege_of_master(&mut self) -> Gpio159wrPrivilegeOfMasterW<Gpio8acSpec> {
        Gpio159wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8acSpec;
impl crate::RegisterSpec for Gpio8acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8ac::R`](R) reader structure"]
impl crate::Readable for Gpio8acSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio8ac::W`](W) writer structure"]
impl crate::Writable for Gpio8acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8AC to value 0xffff_ffff"]
impl crate::Resettable for Gpio8acSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
