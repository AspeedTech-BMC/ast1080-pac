#[doc = "Register `GPIO81C` reader"]
pub type R = crate::R<Gpio81cSpec>;
#[doc = "Register `GPIO81C` writer"]
pub type W = crate::W<Gpio81cSpec>;
#[doc = "Field `GPIO012WrPrivilegeOfMaster` reader - GPIO012 Write Privilege of Master"]
pub type Gpio012wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO012WrPrivilegeOfMaster` writer - GPIO012 Write Privilege of Master"]
pub type Gpio012wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO013WrPrivilegeOfMaster` reader - GPIO013 Write Privilege of Master"]
pub type Gpio013wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO013WrPrivilegeOfMaster` writer - GPIO013 Write Privilege of Master"]
pub type Gpio013wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO014WrPrivilegeOfMaster` reader - GPIO014 Write Privilege of Master"]
pub type Gpio014wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO014WrPrivilegeOfMaster` writer - GPIO014 Write Privilege of Master"]
pub type Gpio014wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO015WrPrivilegeOfMaster` reader - GPIO015 Write Privilege of Master"]
pub type Gpio015wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO015WrPrivilegeOfMaster` writer - GPIO015 Write Privilege of Master"]
pub type Gpio015wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO012 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio012wr_privilege_of_master(&self) -> Gpio012wrPrivilegeOfMasterR {
        Gpio012wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO013 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio013wr_privilege_of_master(&self) -> Gpio013wrPrivilegeOfMasterR {
        Gpio013wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO014 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio014wr_privilege_of_master(&self) -> Gpio014wrPrivilegeOfMasterR {
        Gpio014wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO015 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio015wr_privilege_of_master(&self) -> Gpio015wrPrivilegeOfMasterR {
        Gpio015wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO012 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio012wr_privilege_of_master(&mut self) -> Gpio012wrPrivilegeOfMasterW<Gpio81cSpec> {
        Gpio012wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO013 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio013wr_privilege_of_master(&mut self) -> Gpio013wrPrivilegeOfMasterW<Gpio81cSpec> {
        Gpio013wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO014 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio014wr_privilege_of_master(&mut self) -> Gpio014wrPrivilegeOfMasterW<Gpio81cSpec> {
        Gpio014wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO015 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio015wr_privilege_of_master(&mut self) -> Gpio015wrPrivilegeOfMasterW<Gpio81cSpec> {
        Gpio015wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio81c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio81c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio81cSpec;
impl crate::RegisterSpec for Gpio81cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio81c::R`](R) reader structure"]
impl crate::Readable for Gpio81cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio81c::W`](W) writer structure"]
impl crate::Writable for Gpio81cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO81C to value 0xffff_ffff"]
impl crate::Resettable for Gpio81cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
